#[derive(Debug)]
pub struct PaginatedResult<T> {
    pub photos: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone)]
pub struct PaginationFilter {
    pub page: i64,
    pub per_page: i64,
}
