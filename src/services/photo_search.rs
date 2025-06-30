use crate::{
    models::{PaginatedPhotos, PaginationFilter},
    repositories::{GeoRepository, PhotoFindFilters, PhotoRepository},
    services::embedders::text::TextEmbedder,
};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct PhotoSearchParams {
    pub text: Option<String>,
    pub threshold: Option<f32>,

    pub country: Option<String>,
    pub city: Option<String>,

    pub date_from: Option<String>,
    pub date_to: Option<String>,

    pub page: u32,
    pub per_page: u32,
}

pub struct PhotoSearchService<R1: PhotoRepository, R2: GeoRepository, E: TextEmbedder> {
    photo_repository: R1,
    geo_repository: R2,
    text_embedder: E,
}

impl<R1: PhotoRepository, R2: GeoRepository, E: TextEmbedder> PhotoSearchService<R1, R2, E> {
    pub fn new(photo_repository: R1, geo_repository: R2, text_embedder: E) -> Self {
        Self {
            photo_repository,
            geo_repository,
            text_embedder,
        }
    }

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

        if let Some(country) = search_params.country {
            let country_id = self
                .geo_repository
                .find_country_id_by_name(country)
                .context("Failed to find country id")?;

            find_filters.country_id = country_id;
        }

        if let Some(city) = search_params.city {
            let city_id = self
                .geo_repository
                .find_city_id_by_name(city)
                .context("Failed ton find city id")?;

            find_filters.city_id = city_id;
        }

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
        repositories::{MockGeoRepository, MockPhotoRepository},
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
        let mut service = PhotoSearchService::new(photo_repository, geo_repository, text_embedder);
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

        let mut service = PhotoSearchService::new(repo, geo_repository, text_embedder);
        let result = service.search(PhotoSearchParams::default());

        assert_eq!(result.unwrap_err().to_string(), "Failed to find photos");
    }
}
