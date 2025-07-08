use anyhow::{Context, Result};

use crate::{
    models::{Face, FaceWithPhoto, UpdatedFace},
    repositories::face::repository::FaceRepository,
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

    pub fn update(&mut self, face_id: i32, updated_face: UpdatedFace) -> Result<Face> {
        self.face_repository
            .update_one(face_id, updated_face)
            .context("Failed to update face")
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
