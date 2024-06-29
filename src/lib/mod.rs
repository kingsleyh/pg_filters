use filtering::{Filtering, FilteringRule};
use pagination::Paginate;
use sorting::{SortedColumn, Sorting};

pub mod filtering;
pub mod pagination;
pub mod sorting;

#[derive(Debug, Clone)]
pub struct PgFilters {
    pub pagination: Option<Paginate>,
    pub sorting: Option<Sorting>,
    pub filters: Option<Filtering>,
}

#[derive(Debug, Clone)]
pub struct PaginationOptions {
    pub current_page: i64,
    pub per_page: i64,
    pub per_page_limit: i64,
    pub total_records: i64,
}

impl PgFilters {
    pub fn new(
        pagination: Option<PaginationOptions>,
        sorting_columns: Vec<SortedColumn>,
        filtering_rules: Vec<FilteringRule>,
    ) -> PgFilters {
        let pagination = match pagination {
            Some(pagination) => Some(Paginate::new(
                pagination.current_page,
                pagination.per_page,
                pagination.per_page_limit,
                pagination.total_records,
            )),
            None => None,
        };
        let sorting = Sorting::new(sorting_columns);
        let filters = Filtering::new(filtering_rules);

        PgFilters {
            pagination,
            sorting: Some(sorting),
            filters: Some(filters),
        }
    }

    pub fn sql(&self) -> String {
        let mut sql = "".to_string();

        if let Some(filters) = &self.filters {
            sql.push_str(&filters.sql);
        }

        if let Some(sorting) = &self.sorting {
            sql.push_str(&sorting.sql);
        }

        if let Some(pagination) = &self.pagination {
            sql.push_str(" ");
            sql.push_str(&pagination.sql);
        }

        sql
    }
}
