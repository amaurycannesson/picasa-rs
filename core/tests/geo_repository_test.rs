use picasa_core::repositories::{GeoRepository, PgGeoRepository};
use serial_test::serial;

mod utils;
use utils::get_pool;

#[test]
#[serial]
fn test_find_country_id_by_name_existing() {
    let pool = get_pool();
    let mut geo_repo = PgGeoRepository::new(pool);

    let result = geo_repo.find_country_id_by_name("United States".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(155))
}

#[test]
#[serial]
fn test_find_country_id_by_name_non_existing() {
    let pool = get_pool();
    let mut geo_repo = PgGeoRepository::new(pool);

    let result = geo_repo.find_country_id_by_name("NonExistentCountry".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
#[serial]
fn test_find_city_id_by_name_existing() {
    let pool = get_pool();
    let mut geo_repo = PgGeoRepository::new(pool);

    let result = geo_repo.find_city_id_by_name("New York".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(5128581))
}

#[test]
#[serial]
fn test_find_city_id_by_name_non_existing() {
    let pool = get_pool();
    let mut geo_repo = PgGeoRepository::new(pool);

    let result = geo_repo.find_city_id_by_name("NonExistentCity".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
#[serial]
fn test_find_country_names_by_ids() {
    let pool = get_pool();
    let mut geo_repo = PgGeoRepository::new(pool);

    let ids = vec![1, 2];
    let result = geo_repo.find_country_names_by_ids(ids).unwrap();

    assert_eq!(result[0].id, 1);
    assert_eq!(result[1].id, 2);
}

#[test]
#[serial]
fn test_find_country_names_by_empty_ids() {
    let pool = get_pool();
    let mut geo_repo = PgGeoRepository::new(pool);

    let ids = vec![];
    let result = geo_repo.find_country_names_by_ids(ids);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
#[serial]
fn test_find_city_names_by_ids() {
    let pool = get_pool();
    let mut geo_repo = PgGeoRepository::new(pool);

    let ids = vec![10570, 14256];
    let result = geo_repo.find_city_names_by_ids(ids).unwrap();

    assert_eq!(result[0].id, 10570);
    assert_eq!(result[1].id, 14256);
}

#[test]
#[serial]
fn test_find_city_names_by_empty_ids() {
    let pool = get_pool();
    let mut geo_repo = PgGeoRepository::new(pool);

    let ids = vec![];
    let result = geo_repo.find_city_names_by_ids(ids);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}
