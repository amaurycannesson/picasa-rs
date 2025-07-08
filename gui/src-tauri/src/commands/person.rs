use picasa_core::{
    repositories::{PgFaceRepository, PgPersonRepository},
    services::PersonService,
};
use tauri::State;

use crate::{types::Person, AppState};

#[tauri::command]
#[specta::specta]
pub fn list_persons(state: State<'_, AppState>) -> Result<Vec<Person>, ()> {
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let person_repository = PgPersonRepository::new(state.db_pool.clone());

    let mut person_service = PersonService::new(person_repository, face_repository);

    let persons = person_service.list().unwrap();

    Ok(persons.into_iter().map(Person::from).collect())
}
