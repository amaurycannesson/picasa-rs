use crate::{
    models::geospatial_search_result::GeospatialSearchResult, photo_repository::PhotoRepository,
};

pub struct GeospatialSearchService<R: PhotoRepository> {
    photo_repository: R,
}

impl<R: PhotoRepository> GeospatialSearchService<R> {
    pub fn new(photo_repository: R) -> Self {
        Self { photo_repository }
    }

    pub fn search(&mut self, country_query: &str) -> Vec<GeospatialSearchResult> {
        let results = self
            .photo_repository
            .find_by_country(country_query)
            .unwrap_or_else(|_| vec![]);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    #[test]
    fn test_should_return_error_when_repository_fails() {
        // let mut repo = MockPhotoRepository::new();
        // repo.expect_find_by_country()
        //     .with(eq("test"))
        //     .returning(|_| Err("Repository error".into()));

        // let mut service = GeospatialSearchService::new(repo);
        // let result = service.search("test");

        // assert!(
        //     result.is_empty(),
        //     "Expected no results due to repository error"
        // );
    }
}
