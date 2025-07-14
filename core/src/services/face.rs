use anyhow::{Context, Result};

use crate::{
    models::{Face, FaceWithPhoto, PaginatedFaces, PaginationFilter, UpdatedFace},
    repositories::face::{filters::FaceFindFilters, repository::FaceRepository},
};

pub struct FaceService<FR: FaceRepository> {
    face_repository: FR,
}

impl<FR: FaceRepository> FaceService<FR> {
    pub fn new(face_repository: FR) -> Self {
        Self { face_repository }
    }

    pub fn get_with_photo(&mut self, face_id: i32) -> Result<Option<FaceWithPhoto>> {
        self.face_repository
            .find_with_photo_by_id(face_id)
            .context("Failed to get face with photo")
    }

    pub fn assign_person(&mut self, face_ids: Vec<i32>, person_id: i32) -> Result<Vec<Face>> {
        self.face_repository
            .update_many(
                face_ids,
                UpdatedFace {
                    person_id: Some(Some(person_id)),
                },
            )
            .context("Failed to assign person to faces")
    }

    pub fn list(
        &mut self,
        pagination: PaginationFilter,
        filters: FaceFindFilters,
    ) -> Result<PaginatedFaces> {
        self.face_repository
            .find(pagination, filters)
            .context("Failed to list faces")
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::face::repository::MockFaceRepository;
    use anyhow::anyhow;
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn test_should_return_error_when_repository_fails() {
        let mut face_repository = MockFaceRepository::new();
        face_repository
            .expect_find_with_photo_by_id()
            .with(eq(1))
            .returning(|_| Err(anyhow!("Repository error")));

        let mut service = FaceService::new(face_repository);
        let result = service.get_with_photo(1);

        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to get face with photo"
        );
    }
}
