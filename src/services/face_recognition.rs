use std::time::Instant;

use anyhow::{Context, Result};

use crate::{
    models::{FaceCluster, NewPerson, UpdatedFace},
    repositories::{FaceRepository, PersonRepository},
    utils::progress_reporter::ProgressReporter,
};

#[derive(Debug, Clone)]
pub struct RecognitionConfig {
    pub similarity_threshold: f32,
    pub max_neighbors: i32,
    pub min_cluster_size: i32,
    pub auto_assign_threshold: f32,
    pub min_faces_for_new_person: i32,
}

impl Default for RecognitionConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.6,
            max_neighbors: 20,
            min_cluster_size: 3,
            auto_assign_threshold: 0.7,
            min_faces_for_new_person: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RecognitionAction {
    AutoAssignToExisting {
        person_id: i32,
        person_name: String,
        confidence: f32,
        face_count: i32,
    },
    CreateNewPerson {
        suggested_name: String,
        confidence: f32,
        face_count: i32,
    },
    ManualReview {
        reason: ReviewReason,
        confidence: f32,
        face_count: i32,
        details: String,
    },
    Reject {
        reason: RejectReason,
        face_count: i32,
    },
}

#[derive(Debug, Clone)]
pub enum ReviewReason {
    ConflictingAssignments,
    LowConfidence,
    SmallButPlausibleCluster,
    MixedAssignments,
}

#[derive(Debug, Clone)]
pub enum RejectReason {
    TooFewFaces,
    VeryLowSimilarity,
    HighVarianceCluster,
}

#[derive(Debug)]
pub struct RecognitionResult {
    pub cluster_id: i32,
    pub action: RecognitionAction,
    pub face_ids: Vec<i32>,
    pub photo_paths: Vec<String>,
}

#[derive(Debug)]
pub struct RecognitionSummary {
    pub total_clusters: usize,
    pub auto_assigned_existing: usize,
    pub created_new_persons: usize,
    pub manual_review_needed: usize,
    pub rejected: usize,
    pub total_faces_processed: usize,
    pub results: Vec<RecognitionResult>,
}

pub struct FaceRecognitionService<FR: FaceRepository, PR: PersonRepository, P: ProgressReporter> {
    face_repository: FR,
    person_repository: PR,
    progress_reporter: P,
    config: RecognitionConfig,
}

impl<FR: FaceRepository, PR: PersonRepository, P: ProgressReporter>
    FaceRecognitionService<FR, PR, P>
{
    pub fn new(
        face_repository: FR,
        person_repository: PR,
        progress_reporter: P,
        config: Option<RecognitionConfig>,
    ) -> Self {
        Self {
            face_repository,
            person_repository,
            progress_reporter,
            config: config.unwrap_or_default(),
        }
    }

    /// Main entry point for face recognition.
    pub fn recognize_faces(&mut self, dry_run: bool) -> Result<RecognitionSummary> {
        let start = Instant::now();

        self.progress_reporter
            .set_message("Starting face recognition...".to_string());

        let clusters = self
            .face_repository
            .cluster_similar_faces(
                self.config.similarity_threshold,
                self.config.max_neighbors,
                self.config.min_cluster_size,
            )
            .context("Failed to cluster similar faces")?;

        if clusters.is_empty() {
            self.progress_reporter
                .finish_with_message("No face clusters found.".to_string());
            return Ok(RecognitionSummary {
                total_clusters: 0,
                auto_assigned_existing: 0,
                created_new_persons: 0,
                manual_review_needed: 0,
                rejected: 0,
                total_faces_processed: 0,
                results: vec![],
            });
        }

        self.progress_reporter
            .set_message(format!("Processing {} face clusters...", clusters.len()));

        let mut summary = RecognitionSummary {
            total_clusters: clusters.len(),
            auto_assigned_existing: 0,
            created_new_persons: 0,
            manual_review_needed: 0,
            rejected: 0,
            total_faces_processed: 0,
            results: Vec::new(),
        };

        for cluster in clusters {
            let action = self.determine_action(&cluster)?;
            let face_count = cluster.face_ids.len();

            summary.total_faces_processed += face_count;

            if !dry_run {
                self.execute_action(&cluster, &action).context(format!(
                    "Failed to execute action for cluster {}",
                    cluster.cluster_id
                ))?;
            }

            match &action {
                RecognitionAction::AutoAssignToExisting { .. } => {
                    summary.auto_assigned_existing += 1;
                }
                RecognitionAction::CreateNewPerson { .. } => {
                    summary.created_new_persons += 1;
                }
                RecognitionAction::ManualReview { .. } => {
                    summary.manual_review_needed += 1;
                }
                RecognitionAction::Reject { .. } => {
                    summary.rejected += 1;
                }
            }

            summary.results.push(RecognitionResult {
                cluster_id: cluster.cluster_id,
                action,
                face_ids: cluster.face_ids,
                photo_paths: cluster.photo_paths,
            });
        }

        let duration = start.elapsed();
        let mode_str = if dry_run { " (dry run)" } else { "" };
        self.progress_reporter.finish_with_message(format!(
            "✓ Processed {} clusters in {:.2?}{}",
            summary.total_clusters, duration, mode_str
        ));

        Ok(summary)
    }

    /// Determine what action to take for a face cluster
    fn determine_action(&mut self, cluster: &FaceCluster) -> Result<RecognitionAction> {
        let confidence = self.calculate_cluster_confidence(cluster);

        let assigned_person_ids: Vec<i32> = cluster
            .person_ids
            .iter()
            .filter(|&&id| id > 0)
            .copied()
            .collect();
        let unique_persons = assigned_person_ids.len();
        let total_unassigned = cluster.face_ids_without_person.len() as i32;

        match (unique_persons, total_unassigned) {
            // Case 1: Multiple different persons assigned - conflict!
            (2.., _) => Ok(RecognitionAction::ManualReview {
                reason: ReviewReason::ConflictingAssignments,
                confidence,
                face_count: cluster.face_count,
                details: format!(
                    "Cluster contains faces assigned to {} different persons",
                    unique_persons
                ),
            }),

            // Case 2: Single person already assigned - extend assignment
            (1, unassigned_count) if unassigned_count > 0 => {
                let person_id = assigned_person_ids[0];

                if confidence >= self.config.auto_assign_threshold {
                    Ok(RecognitionAction::AutoAssignToExisting {
                        person_id,
                        person_name: format!("Person {}", person_id),
                        confidence,
                        face_count: unassigned_count,
                    })
                } else {
                    Ok(RecognitionAction::ManualReview {
                        reason: ReviewReason::LowConfidence,
                        confidence,
                        face_count: cluster.face_count,
                        details: format!(
                            "Low confidence ({:.2}) for extending assignment to existing person",
                            confidence
                        ),
                    })
                }
            }

            // Case 3: No assigned persons - new person or review
            (0, unassigned_count) => {
                if unassigned_count < self.config.min_faces_for_new_person {
                    Ok(RecognitionAction::Reject {
                        reason: RejectReason::TooFewFaces,
                        face_count: cluster.face_count,
                    })
                } else if confidence >= self.config.auto_assign_threshold {
                    Ok(RecognitionAction::CreateNewPerson {
                        suggested_name: format!("Unknown Person #{}", cluster.cluster_id),
                        confidence,
                        face_count: unassigned_count,
                    })
                } else {
                    Ok(RecognitionAction::ManualReview {
                        reason: ReviewReason::SmallButPlausibleCluster,
                        confidence,
                        face_count: cluster.face_count,
                        details: format!(
                            "Moderate confidence ({:.2}) cluster needs review",
                            confidence
                        ),
                    })
                }
            }

            // Fallback for unexpected states
            _ => Ok(RecognitionAction::ManualReview {
                reason: ReviewReason::MixedAssignments,
                confidence,
                face_count: cluster.face_count,
                details: "Unexpected cluster state".to_string(),
            }),
        }
    }

    /// Calculate confidence score for a cluster
    fn calculate_cluster_confidence(&self, cluster: &FaceCluster) -> f32 {
        // Weighted combination of factors
        let avg_weight = 0.4;
        let min_weight = 0.3;
        let size_weight = 0.3;

        // Normalize face count (more faces = higher confidence, with diminishing returns)
        let size_factor = (cluster.face_count as f32).ln() / 10.0_f32.ln(); // Log scale, max at 10 faces
        let size_factor = size_factor.min(1.0);

        let confidence = (cluster.avg_similarity_score * avg_weight)
            + (cluster.min_similarity_score * min_weight)
            + (size_factor * size_weight);

        confidence.min(1.0).max(0.0)
    }

    /// Execute the determined action
    fn execute_action(&mut self, cluster: &FaceCluster, action: &RecognitionAction) -> Result<()> {
        match action {
            RecognitionAction::AutoAssignToExisting { person_id, .. } => {
                for &face_id in &cluster.face_ids_without_person {
                    self.face_repository.update_one(
                        face_id,
                        UpdatedFace {
                            person_id: Some(Some(*person_id)),
                            ..Default::default()
                        },
                    )?;
                }
            }
            RecognitionAction::CreateNewPerson { suggested_name, .. } => {
                let new_person = NewPerson {
                    name: suggested_name.clone(),
                };
                let person = self.person_repository.insert_one(new_person)?;

                // Assign all unassigned faces to the new person
                for &face_id in &cluster.face_ids_without_person {
                    self.face_repository.update_one(
                        face_id,
                        UpdatedFace {
                            person_id: Some(Some(person.id)),
                            ..Default::default()
                        },
                    )?;
                }
            }
            RecognitionAction::ManualReview { .. } | RecognitionAction::Reject { .. } => {
                // No action needed - these are for reporting only
            }
        }

        Ok(())
    }
}
