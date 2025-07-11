use picasa_core::database::{self, DbPool};
#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

pub mod commands;
pub mod services;
pub mod types;

pub struct AppState {
    pub db_pool: DbPool,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let db_pool = database::create_pool()?;

        Ok(Self { db_pool })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::photo::search_photos,
        commands::photo::load_photo,
        commands::photo::get_search_options,
        commands::face::get_pending_manual_reviews,
        commands::face::load_face_image,
        commands::person::create_person_from_faces,
        commands::person::list_persons,
        commands::person::get_person,
    ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    let app_state = AppState::new().expect("Failed to create app state");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
