#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct SortedColumn {
    pub column: String,
    pub order: SortOrder,
}

#[derive(Debug, Clone)]
pub struct Sorting {
    pub columns: Vec<SortedColumn>,
    pub sql: String,
}

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
