use crate::models::url::{Url, UrlJson};
use crate::schema::url;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::{Local, NaiveDateTime};

#[derive(Insertable)]
#[diesel(table_name = url)]
struct NewUrl<'a> {
    short_url: &'a str,
    long_url: &'a str,
    created_on: &'a NaiveDateTime,
}

pub fn create(conn: &mut PgConnection, short_url: &str, long_url: &str) -> UrlJson {
    let new_url = &NewUrl {
        short_url,
        long_url,
        created_on: &Local::now().naive_utc(),
    };
    diesel::insert_into(url::table)
        .values(new_url)
        .get_result::<Url>(conn)
        .expect("Failed to add url")
        .attach()
}

pub fn find_by_long_url(conn: &mut PgConnection, long_url: &str) -> bool {
    let result = url::table
        .filter(url::long_url.eq(long_url))
        .limit(1)
        .load::<Url>(conn)
        .expect("Error loading url");
    if result.len() > 0 {
        return true
    }
    false
}

pub fn get_next_id(conn: &mut PgConnection) -> Result<i32, diesel::result::Error>{
    let result = url::table
        .order(url::id.desc())
        .first::<Url>(conn);

    match result {
        Ok(last_entry) => {
            Ok(last_entry.id)
        }
        Err(error) => {
            Err(error)
        }
    }
}


pub fn find(conn: &mut PgConnection, short_url: &str) -> Result<String, diesel::result::Error> {
    let url: Url = url::table
        .filter(url::short_url.eq(short_url))
        .first::<Url>(conn)?;
    return Ok(String::from(url.attach().long_url));
}