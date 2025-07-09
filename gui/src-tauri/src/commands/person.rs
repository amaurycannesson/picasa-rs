use picasa_core::{
    models::NewPerson,
    repositories::{PgFaceRepository, PgPersonRepository},
    services::PersonService,
};
use tauri::State;

use crate::{types::Person, AppState};

#[tauri::command]
#[specta::specta]
pub async fn list_persons(state: State<'_, AppState>) -> Result<Vec<Person>, String> {
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let person_repository = PgPersonRepository::new(state.db_pool.clone());

    let mut person_service = PersonService::new(person_repository, face_repository);

    person_service
        .list()
        .map(|p| p.into_iter().map(Person::from).collect())
        .map_err(|e| format!("Failed to list people: {}", e))
}

#[tauri::command]
#[specta::specta]
pub async fn create_person_from_faces(
    person_name: &str,
    face_ids: Vec<i32>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let person_repository = PgPersonRepository::new(state.db_pool.clone());

    let mut person_service = PersonService::new(person_repository, face_repository);

    person_service
        .create_from_faces(
            NewPerson {
                name: person_name.to_string(),
            },
            face_ids,
        )
        .map_err(|e| format!("Failed to create person from faces: {}", e))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_person(id: i32, state: State<'_, AppState>) -> Result<Person, String> {
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let person_repository = PgPersonRepository::new(state.db_pool.clone());

    let mut person_service = PersonService::new(person_repository, face_repository);

    person_service
        .get(id)
        .map(Person::from)
        .map_err(|e| format!("Failed to get person: {}", e))
}
