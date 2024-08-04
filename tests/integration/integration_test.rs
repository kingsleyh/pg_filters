// use pg_filters::{filtering::{FilteringRule, ColumnName}, sorting::{SortedColumn}, PaginationOptions, PgFilters};
// use pg_filters::FilteringOptions;
// use testcontainers_modules::{postgres, testcontainers::runners::SyncRunner};
// use testcontainers_modules::postgres::Postgres;
// #[test]
// fn test_something_works() {
//
//     let container = postgres::Postgres::default().start().unwrap();
//
//
//     // prepare connection string
//     // let connection_string = &format!(
//     //     "postgres://postgres:postgres@127.0.0.1:{}/postgres",
//     //     container.get_host_port_ipv4(5432)?
//     // );
//     // container is up, you can use it
//     // let mut conn = postgres::Client::connect(connection_string, postgres::NoTls).unwrap();
//     // let rows = conn.query("SELECT 1 + 1", &[]).unwrap();
//     // assert_eq!(rows.len(), 1);
//
//     let filters = PgFilters::new(
//        Some(PaginationOptions {
//            current_page: 1,
//            per_page: 10,
//            per_page_limit: 10,
//            total_records: 1000,
//        }),
//        vec![
//            SortedColumn::new("age", "desc"),
//            SortedColumn::new("name", "asc"),
//        ],
//        Some(FilteringOptions::new(vec![
//            FilteringRule::new("where", ColumnName::String("name"), "=", "John"),
//            FilteringRule::new("or", ColumnName::Int("age"), ">", "18"),
//        ])),
//     );
//
//     println!("{:?}", filters.sql());
// }