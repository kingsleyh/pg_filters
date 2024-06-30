//! Sorting module
//!
//! This module contains the Sorting struct and its implementation
//!
//! The Sorting struct is used to generate the ORDER BY clause in SQL queries
//!
//! # Example
//!
//! ```rust
//! use pg_filters::sorting::{SortedColumn, Sorting};
//!
//! let sorting = Sorting::new(vec![
//!    SortedColumn::new("name", "asc".to_string()),
//!    SortedColumn::new("age", "desc".to_string()),
//! ]);
//!
//! assert_eq!(sorting.columns.len(), 2);
//! assert_eq!(sorting.sql, " ORDER BY age DESC, name ASC");
//! ```
//!

/// SortOrder enum
///
/// This enum is used to specify the sorting order of a column
///
/// # Example
///
/// ```rust
/// use pg_filters::sorting::SortOrder;
///
/// let order = SortOrder::Asc;
///
/// assert_eq!(order, SortOrder::Asc);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// SortedColumn struct
///
/// This struct is used to hold the column name and sorting order
#[derive(Debug, Clone)]
pub struct SortedColumn {
    /// Column name
    pub column: String,
    /// Sorting order
    pub order: SortOrder,
}

/// New function for SortedColumn
///
/// This function takes a column name and sorting order and returns a SortedColumn struct
///
/// # Example
///
/// ```rust
/// use pg_filters::sorting::SortedColumn;
/// use pg_filters::sorting::SortOrder;
///
/// let column = SortedColumn::new("name", "asc".to_string());
///
/// assert_eq!(column.column, "name");
/// assert_eq!(column.order, SortOrder::Asc);
/// ```
///
impl SortedColumn {
    pub fn new(column: &str, order: String) -> SortedColumn {
        let order = match order.to_lowercase().as_str() {
            "asc" => SortOrder::Asc,
            "desc" => SortOrder::Desc,
            _ => SortOrder::Asc,
        };
        SortedColumn {
            column: column.to_string(),
            order,
        }
    }
}

/// Sorting struct
///
/// This struct is used to generate the ORDER BY clause in SQL queries
///
/// # Example
///
/// ```rust
/// use pg_filters::sorting::{SortedColumn, Sorting};
///
/// let sorting = Sorting::new(vec![
///    SortedColumn::new("name", "asc".to_string()),
///    SortedColumn::new("age", "desc".to_string()),
/// ]);
///
/// assert_eq!(sorting.columns.len(), 2);
/// assert_eq!(sorting.sql, " ORDER BY age DESC, name ASC");
/// ```
///
#[derive(Debug, Clone)]
pub struct Sorting {
    /// Vector of SortedColumn structs
    pub columns: Vec<SortedColumn>,
    /// SQL ORDER BY clause
    pub sql: String,
}

/// New function for Sorting
///
/// This function takes a vector of SortedColumn structs and returns a Sorting struct
///
/// # Example
///
/// ```rust
/// use pg_filters::sorting::{SortedColumn, Sorting};
///
/// let sorting = Sorting::new(vec![
///    SortedColumn::new("name", "asc".to_string()),
///    SortedColumn::new("age", "desc".to_string()),
/// ]);
///
/// assert_eq!(sorting.columns.len(), 2);
/// assert_eq!(sorting.sql, " ORDER BY age DESC, name ASC");
/// ```
///
impl Sorting {
    pub fn new(columns: Vec<SortedColumn>) -> Sorting {
        let mut columns = columns;
        columns.sort_by(|a, b| a.column.cmp(&b.column));
        columns.dedup_by(|a, b| a.column == b.column);

        let mut sql = if columns.len() > 0 {
            " ORDER BY ".to_string()
        } else {
            "".to_string()
        };
        let mut first = true;
        for column in columns.iter() {
            if first {
                first = false;
            } else {
                sql.push_str(", ");
            }
            match column.order {
                SortOrder::Asc => {
                    sql.push_str(&column.column);
                    sql.push_str(" ASC");
                }
                SortOrder::Desc => {
                    sql.push_str(&column.column);
                    sql.push_str(" DESC");
                }
            }
        }
        Sorting { columns, sql }
    }
}
