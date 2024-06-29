#[derive(Debug, Clone)]
pub struct Pagination {
    pub current_page: i64,
    pub previous_page: i64,
    pub next_page: i64,
    pub total_pages: i64,
    pub per_page: i64,
    pub total_records: i64,
}

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

#[derive(Debug, Clone)]
pub struct Paginate {
    pub pagination: Pagination,
    pub sql: String,
}

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
