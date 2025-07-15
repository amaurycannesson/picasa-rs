use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use crate::{
    models::{CityName, CountryName, Face, PaginatedPhotos, PaginationFilter, Person, Photo},
    repositories::{
        FaceRepository, FindPersonFilters, GeoRepository, PersonMatchMode, PersonRepository,
        PhotoFindFilters, PhotoRepository, face::filters::FaceFindFilters,
    },
    services::embedders::text::TextEmbedder,
};

#[derive(Debug)]
pub struct FaceWithPerson {
    pub face: Face,
    pub person: Option<Person>,
}

#[derive(Debug)]
pub struct PhotoWithFacesAndPeople {
    pub photo: Photo,
    pub faces: Vec<FaceWithPerson>,
}

#[derive(Debug)]
pub struct PhotoSearchOptions {
    pub cities: Vec<CityName>,
    pub countries: Vec<CountryName>,
    pub persons: Vec<Person>,
}

#[derive(Default)]
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

pub struct PhotoSearchService<
    PR: PhotoRepository,
    GR: GeoRepository,
    PR2: PersonRepository,
    FR: FaceRepository,
    E: TextEmbedder,
> {
    photo_repository: PR,
    geo_repository: GR,
    person_repository: PR2,
    face_repository: FR,
    text_embedder: E,
}

impl<
    PR: PhotoRepository,
    GR: GeoRepository,
    PR2: PersonRepository,
    FR: FaceRepository,
    E: TextEmbedder,
> PhotoSearchService<PR, GR, PR2, FR, E>
{
    pub fn new(
        photo_repository: PR,
        geo_repository: GR,
        person_repository: PR2,
        face_repository: FR,
        text_embedder: E,
    ) -> Self {
        Self {
            photo_repository,
            geo_repository,
            person_repository,
            face_repository,
            text_embedder,
        }
    }

    /// Returns available search options (cities, countries, persons) based on existing photos.
    pub fn get_search_options(&mut self) -> Result<PhotoSearchOptions> {
        let country_ids = self.photo_repository.find_country_ids()?;
        let city_ids = self.photo_repository.find_city_ids()?;
        let person_ids = self.photo_repository.find_person_ids()?;

        let countries = if !country_ids.is_empty() {
            self.geo_repository.find_country_names_by_ids(country_ids)?
        } else {
            Vec::new()
        };

        let cities = if !city_ids.is_empty() {
            self.geo_repository.find_city_names_by_ids(city_ids)?
        } else {
            Vec::new()
        };

        let persons = if !person_ids.is_empty() {
            self.person_repository.find_many(FindPersonFilters {
                ids: Some(person_ids),
            })?
        } else {
            Vec::new()
        };

        Ok(PhotoSearchOptions {
            cities,
            countries,
            persons,
        })
    }

    /// Gets a single photo with its faces and associated people by photo ID.
    pub fn get_photo_with_faces_and_people(
        &mut self,
        id: i32,
    ) -> Result<Option<PhotoWithFacesAndPeople>> {
        let photo = match self
            .photo_repository
            .find_by_id(id)
            .context("Failed to find photo by id")?
        {
            Some(photo) => photo,
            None => return Ok(None),
        };

        let faces = self
            .face_repository
            .find(
                PaginationFilter {
                    page: 1,
                    per_page: 1000,
                },
                FaceFindFilters {
                    photo_id: Some(id),
                    ..Default::default()
                },
            )
            .context("Failed to find faces for photo")?
            .items;

        let person_ids: Vec<i32> = faces
            .iter()
            .filter_map(|face| face.person_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let persons = if !person_ids.is_empty() {
            self.person_repository
                .find_many(FindPersonFilters {
                    ids: Some(person_ids),
                })
                .context("Failed to find persons for faces")?
        } else {
            Vec::new()
        };

        let mut person_map: std::collections::HashMap<i32, Person> = persons
            .into_iter()
            .map(|person| (person.id, person))
            .collect();

        let faces_with_people: Vec<FaceWithPerson> = faces
            .into_iter()
            .map(|face| {
                let person = face.person_id.and_then(|id| person_map.remove(&id));
                FaceWithPerson { face, person }
            })
            .collect();

        Ok(Some(PhotoWithFacesAndPeople {
            photo,
            faces: faces_with_people,
        }))
    }

    /// Searches for photos based on the provided search parameters.
    pub fn search(&mut self, search_params: PhotoSearchParams) -> Result<PaginatedPhotos> {
        let mut find_filters = PhotoFindFilters::default();

        if let Some(text) = search_params.text {
            let text_embedding = self
                .text_embedder
                .embed(&text)
                .context("Failed to create text embedding")?;

            find_filters.text_embedding = Some(text_embedding);
        }

        find_filters.threshold = search_params.threshold;

        let country_id = match (search_params.country_id, search_params.country) {
            (Some(id), _) => Some(id),
            (None, Some(name)) => self.geo_repository.find_country_id_by_name(name)?,
            _ => None,
        };
        find_filters.country_id = country_id;

        let city_id = match (search_params.city_id, search_params.city) {
            (Some(id), _) => Some(id),
            (None, Some(name)) => self.geo_repository.find_city_id_by_name(name)?,
            _ => None,
        };
        find_filters.city_id = city_id;

        if let Some(date_from) = search_params.date_from {
            let parsed_date = date_from
                .parse::<DateTime<Utc>>()
                .context("Failed to parse date_from")?;
            find_filters.date_from = Some(parsed_date);
        }

        if let Some(date_to) = search_params.date_to {
            let parsed_date = date_to
                .parse::<DateTime<Utc>>()
                .context("Failed to parse date_to")?;
            find_filters.date_to = Some(parsed_date);
        }

        find_filters.person_ids = search_params.person_ids;
        find_filters.person_match_mode = search_params.person_match_mode;

        let pagination_filter = PaginationFilter {
            page: search_params.page as i64,
            per_page: search_params.per_page as i64,
        };

        self.photo_repository
            .find(pagination_filter, find_filters)
            .context("Failed to find photos")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        repositories::{
            face::repository::MockFaceRepository, geo::MockGeoRepository,
            person::repository::MockPersonRepository, photo::repository::MockPhotoRepository,
        },
        services::embedders::text::MockTextEmbedder,
    };

    use super::*;
    use anyhow::anyhow;
    use mockall::predicate::eq;

    #[test]
    fn test_should_return_error_when_embedder_fails() {
        let mut text_embedder = MockTextEmbedder::new();
        text_embedder
            .expect_embed()
            .with(eq("test"))
            .returning(|_| Err(anyhow!("Embedding error")));

        let photo_repository = MockPhotoRepository::new();
        let geo_repository = MockGeoRepository::new();
        let person_repository = MockPersonRepository::new();
        let face_repository = MockFaceRepository::new();
        let mut service = PhotoSearchService::new(
            photo_repository,
            geo_repository,
            person_repository,
            face_repository,
            text_embedder,
        );
        let result = service.search(PhotoSearchParams {
            text: Some("test".to_string()),
            ..PhotoSearchParams::default()
        });

        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to create text embedding"
        );
    }

    #[test]
    fn test_should_return_error_when_repository_fails() {
        let text_embedder = MockTextEmbedder::new();
        let geo_repository = MockGeoRepository::new();
        let mut repo = MockPhotoRepository::new();
        repo.expect_find()
            .returning(|_, __| Err(anyhow!("Repository error")));

        let person_repository = MockPersonRepository::new();
        let face_repository = MockFaceRepository::new();
        let mut service = PhotoSearchService::new(
            repo,
            geo_repository,
            person_repository,
            face_repository,
            text_embedder,
        );
        let result = service.search(PhotoSearchParams::default());

        assert_eq!(result.unwrap_err().to_string(), "Failed to find photos");
    }
}
