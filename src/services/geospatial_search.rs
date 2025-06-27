use crate::{
    models::geospatial_search_result::GeospatialSearchResult, photo_repository::PhotoRepository,
};
use anyhow::{Context, Result};

pub struct GeospatialSearchService<R: PhotoRepository> {
    photo_repository: R,
}

impl<R: PhotoRepository> GeospatialSearchService<R> {
    pub fn new(photo_repository: R) -> Self {
        Self { photo_repository }
    }

    pub fn search(&mut self, country_query: &str) -> Result<Vec<GeospatialSearchResult>> {
        self.photo_repository
            .find_by_country(country_query)
            .context("Failed to search by country")
    }
}

#[cfg(test)]
mod tests {
    use crate::photo_repository::MockPhotoRepository;

    use super::*;
    use anyhow::anyhow;
    use mockall::predicate::eq;

    #[test]
    fn test_should_return_error_when_repository_fails() {
        let mut repo = MockPhotoRepository::new();
        repo.expect_find_by_country()
            .with(eq("test"))
            .returning(|_| Err(anyhow!("Repository error")));

        let mut service = GeospatialSearchService::new(repo);
        let result = service.search("test");

        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to search by country"
        );
    }
}
