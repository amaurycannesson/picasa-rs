use anyhow::{Context, Result};

use crate::{
    models::{NewPerson, Person, UpdatedFace},
    repositories::{FaceRepository, PersonRepository},
};

pub struct PersonService<PR: PersonRepository, FR: FaceRepository> {
    person_repository: PR,
    face_repository: FR,
}

impl<PR: PersonRepository, FR: FaceRepository> PersonService<PR, FR> {
    pub fn new(person_repository: PR, face_repository: FR) -> Self {
        Self {
            person_repository,
            face_repository,
        }
    }

    pub fn create_from_faces(
        &mut self,
        new_person: NewPerson,
        face_ids: Vec<i32>,
    ) -> Result<Person> {
        let new_person = self
            .person_repository
            .insert_one(new_person)
            .context("Failed to create person")?;

        self.face_repository
            .update_many(
                face_ids,
                UpdatedFace {
                    person_id: Some(Some(new_person.id)),
                },
            )
            .context("Failed to create faces")?;

        Ok(new_person)
    }

    pub fn list(&mut self) -> Result<Vec<Person>> {
        self.person_repository
            .find_many()
            .context("Failed to retrieve persons")
    }

    pub fn get(&mut self, id: i32) -> Result<Person> {
        self.person_repository
            .find_by_id(id)
            .context("Failed to retrieve person")
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::{
        face::repository::MockFaceRepository, person::repository::MockPersonRepository,
    };
    use anyhow::anyhow;

    use super::*;

    #[test]
    fn test_should_return_error_when_repository_fails() {
        let mut person_repository = MockPersonRepository::new();
        let face_repository = MockFaceRepository::new();
        let name = "John Doe";
        let new_person = NewPerson {
            name: name.to_string(),
        };
        let face_ids = vec![1, 2, 3];

        person_repository
            .expect_insert_one()
            .withf(move |p: &NewPerson| p.name == name)
            .returning(|_| Err(anyhow!("Repository error")));

        let mut service = PersonService::new(person_repository, face_repository);
        let result = service.create_from_faces(new_person, face_ids);

        assert_eq!(result.unwrap_err().to_string(), "Failed to create person");
    }
}
