//! Pagination struct and methods
//! 
//! Pagination struct holds the current page, previous page, next page, total pages, number of records per page and total records
//! 
//! Pagination struct has a new method that takes the current page, number of records per page, total pages and total records and returns a Pagination struct
//!
//! Paginate struct holds the pagination struct and the SQL LIMIT and OFFSET clause

/// Pagination struct
/// 
/// Pagination struct holds the current page, previous page, next page, total pages, number of records per page and total records
/// 
/// Pagination struct has a new method that takes the current page, number of records per page, total pages and total records and returns a Pagination struct
/// 
/// # Example
/// 
/// ```rust
/// use pg_filters::pagination::Pagination;
/// 
/// let pagination = Pagination::new(1, 10, 100, 1000);
/// assert_eq!(pagination.current_page, 1);
/// assert_eq!(pagination.previous_page, 1);
/// assert_eq!(pagination.next_page, 2);
/// assert_eq!(pagination.total_pages, 100);
/// assert_eq!(pagination.per_page, 10);
/// assert_eq!(pagination.total_records, 1000);
/// ```
/// 
#[derive(Debug, Clone)] 
pub struct Pagination {
    pub current_page: i64,
    pub previous_page: i64,
    pub next_page: i64,
    pub total_pages: i64,
    pub per_page: i64,
    pub total_records: i64,
}

/// New function for Pagination
/// 
/// Pagination struct has a new method that takes the current page, number of records per page, total pages and total records and returns a Pagination struct
/// 
/// # Example
/// 
/// ```rust
/// use pg_filters::pagination::Pagination;
/// 
/// let pagination = Pagination::new(1, 10, 100, 1000);
/// assert_eq!(pagination.current_page, 1);
/// assert_eq!(pagination.previous_page, 1);
/// assert_eq!(pagination.next_page, 2);
/// assert_eq!(pagination.total_pages, 100);
/// assert_eq!(pagination.per_page, 10);
/// assert_eq!(pagination.total_records, 1000);
/// ```
/// 
impl Pagination {
    pub fn new(
        current_page: i64,
        per_page: i64,
        total_pages: i64,
        total_records: i64,
    ) -> Pagination {
        let previous_page = if current_page > 1 {
            current_page - 1
        } else {
            1
        };
        let next_page = if current_page < total_pages {
            current_page + 1
        } else {
            total_pages
        };

        Pagination {
            current_page,
            previous_page,
            next_page,
            total_pages,
            per_page,
            total_records,
        }
    }
}

/// Paginate struct
#[derive(Debug, Clone)]
pub struct Paginate {
    /// Pagination struct
    pub pagination: Pagination,
    /// SQL LIMIT and OFFSET clause
    pub sql: String,
}

/// New function for Paginate
/// 
/// Paginate struct has a new method that takes the current page, number of records per page, maximum number of records per page and total records and returns a Paginate struct
/// 
/// # Example
/// 
/// ```rust
/// use pg_filters::pagination::Paginate;
/// 
/// let paginate = Paginate::new(1, 10, 10, 1000);
/// assert_eq!(paginate.pagination.current_page, 1);
/// assert_eq!(paginate.pagination.previous_page, 1);
/// assert_eq!(paginate.pagination.next_page, 2);
/// assert_eq!(paginate.pagination.total_pages, 100);
/// assert_eq!(paginate.pagination.per_page, 10);
/// assert_eq!(paginate.pagination.total_records, 1000);
/// assert_eq!(paginate.sql, "LIMIT 10 OFFSET 0");
/// ```
/// 
impl Paginate {
    pub fn new(
        current_page: i64,
        per_page: i64,
        per_page_limit: i64,
        total_records: i64,
    ) -> Paginate {
        let per_page = per_page;
        let per_page = if per_page > per_page_limit {
            per_page_limit
        } else {
            per_page
        };

        let total_pages = (total_records as f64 / per_page as f64).ceil() as i64;

        let current_page = if current_page < 1 { 1 } else { current_page };
        let current_page = if current_page > total_pages {
            total_pages
        } else {
            current_page
        };

        let limit = per_page;
        let offset = (limit * current_page) - limit;

        let pagination = Pagination::new(current_page, per_page, total_pages, total_records);
        let sql = format!("LIMIT {} OFFSET {}", limit, offset);

        Paginate { pagination, sql }
    }
}
