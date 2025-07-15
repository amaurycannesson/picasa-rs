use picasa_core::{models, repositories, services};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::types::{face::Face, CityName, CountryName, Person};

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Photo {
    pub id: i32,
    pub path: String,
    pub file_name: String,
    pub file_size: i64,
    pub created_at: String,
    pub modified_at: String,
    pub hash: Option<String>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub orientation: Option<i32>,
    pub date_taken_local: Option<String>,
    pub date_taken_utc: Option<String>,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
    pub face_detection_completed: bool,
    pub country_id: Option<i32>,
    pub city_id: Option<i32>,
}

impl From<models::Photo> for Photo {
    fn from(core_photo: models::Photo) -> Self {
        Self {
            id: core_photo.id,
            path: core_photo.path,
            file_name: core_photo.file_name,
            file_size: core_photo.file_size as i64,
            created_at: core_photo.created_at.to_rfc3339(),
            modified_at: core_photo.modified_at.to_rfc3339(),
            hash: core_photo.hash,
            camera_make: core_photo.camera_make,
            camera_model: core_photo.camera_model,
            lens_model: core_photo.lens_model,
            orientation: core_photo.orientation,
            date_taken_local: core_photo.date_taken_local.map(|dt| dt.to_string()),
            date_taken_utc: core_photo.date_taken_utc.map(|dt| dt.to_rfc3339()),
            image_width: core_photo.image_width,
            image_height: core_photo.image_height,
            face_detection_completed: core_photo.face_detection_completed,
            country_id: core_photo.country_id,
            city_id: core_photo.city_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct PaginatedPhotos {
    pub items: Vec<Photo>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl From<models::PaginatedPhotos> for PaginatedPhotos {
    fn from(paginated_photos: models::PaginatedPhotos) -> Self {
        Self {
            items: paginated_photos
                .items
                .into_iter()
                .map(Photo::from)
                .collect(),
            total: paginated_photos.total,
            page: paginated_photos.page,
            per_page: paginated_photos.per_page,
            total_pages: paginated_photos.total_pages,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub enum PersonMatchMode {
    Any,
    All,
}

impl Default for PersonMatchMode {
    fn default() -> Self {
        PersonMatchMode::Any
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Default)]
pub struct PhotoSearchParams {
    pub text: Option<String>,
    pub threshold: Option<f32>,

    pub country: Option<String>,
    pub country_id: Option<i32>,
    pub city: Option<String>,
    pub city_id: Option<i32>,

    pub date_from: Option<String>,
    pub date_to: Option<String>,

    pub person_ids: Option<Vec<i32>>,
    pub person_match_mode: Option<PersonMatchMode>,

    pub page: u32,
    pub per_page: u32,
}

impl From<services::PhotoSearchParams> for PhotoSearchParams {
    fn from(photo_search_params: services::PhotoSearchParams) -> Self {
        Self {
            text: photo_search_params.text,
            threshold: photo_search_params.threshold,
            country: photo_search_params.country,
            country_id: photo_search_params.country_id,
            city: photo_search_params.city,
            city_id: photo_search_params.city_id,
            date_from: photo_search_params.date_from,
            date_to: photo_search_params.date_to,
            person_ids: photo_search_params.person_ids,
            person_match_mode: match photo_search_params.person_match_mode {
                Some(mode) => match mode {
                    repositories::PersonMatchMode::All => Some(PersonMatchMode::All),
                    repositories::PersonMatchMode::Any => Some(PersonMatchMode::Any),
                },
                None => None,
            },
            page: photo_search_params.page,
            per_page: photo_search_params.per_page,
        }
    }
}

impl From<PhotoSearchParams> for services::PhotoSearchParams {
    fn from(photo_search_params: PhotoSearchParams) -> Self {
        Self {
            text: photo_search_params.text,
            threshold: photo_search_params.threshold,
            country: photo_search_params.country,
            country_id: photo_search_params.country_id,
            city: photo_search_params.city,
            city_id: photo_search_params.city_id,
            date_from: photo_search_params.date_from,
            date_to: photo_search_params.date_to,
            person_ids: photo_search_params.person_ids,
            person_match_mode: match photo_search_params.person_match_mode {
                Some(mode) => match mode {
                    PersonMatchMode::All => Some(repositories::PersonMatchMode::All),
                    PersonMatchMode::Any => Some(repositories::PersonMatchMode::Any),
                },
                None => None,
            },
            page: photo_search_params.page,
            per_page: photo_search_params.per_page,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct PhotoSearchOptions {
    pub cities: Vec<CityName>,
    pub countries: Vec<CountryName>,
    pub persons: Vec<Person>,
}

impl From<services::photo_search::PhotoSearchOptions> for PhotoSearchOptions {
    fn from(options: services::photo_search::PhotoSearchOptions) -> Self {
        Self {
            cities: options.cities.into_iter().map(CityName::from).collect(),
            countries: options
                .countries
                .into_iter()
                .map(CountryName::from)
                .collect(),
            persons: options.persons.into_iter().map(Person::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct FaceWithPerson {
    pub face: Face,
    pub person: Option<Person>,
}

impl From<services::photo_search::FaceWithPerson> for FaceWithPerson {
    fn from(core_face_with_person: services::photo_search::FaceWithPerson) -> Self {
        Self {
            face: Face::from(core_face_with_person.face),
            person: core_face_with_person.person.map(Person::from),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct PhotoWithFacesAndPeople {
    pub photo: Photo,
    pub faces: Vec<FaceWithPerson>,
}

impl From<services::photo_search::PhotoWithFacesAndPeople> for PhotoWithFacesAndPeople {
    fn from(core_photo_with_faces: services::photo_search::PhotoWithFacesAndPeople) -> Self {
        Self {
            photo: Photo::from(core_photo_with_faces.photo),
            faces: core_photo_with_faces
                .faces
                .into_iter()
                .map(FaceWithPerson::from)
                .collect(),
        }
    }
}
