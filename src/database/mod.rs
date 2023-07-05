pub mod url;

#[database("diesel_postgres_pool")]
pub struct Db(diesel::PgConnection);