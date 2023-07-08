use crate::config::DATE_FORMAT;
use serde::Serialize;
use chrono::{NaiveDateTime, DateTime, Utc, NaiveDate, NaiveTime};

#[derive(Queryable,Debug)]
pub struct Url {
    pub id: i32,
    pub short_url: String,
    pub long_url: String,
    pub created_on: NaiveDateTime,
}


fn convert_string_to_datetime(string: String) -> Result<DateTime<Utc>, chrono::ParseError> {
    if string.is_empty() {
        let naive_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let datetime = DateTime::<Utc>::from_utc(naive_datetime, Utc);
        return Ok(datetime);
    }

    let naive_datetime = NaiveDateTime::parse_from_str(&string, DATE_FORMAT)?;
    let datetime = DateTime::<Utc>::from_utc(naive_datetime, Utc);
    Ok(datetime)
}

impl Url {
    pub fn attach(self) -> UrlJson {
        let created_on: String = self.created_on.format(DATE_FORMAT).to_string();
        let created_on_datetime = convert_string_to_datetime(created_on).unwrap();
        UrlJson {
            id: self.id,
            short_url: self.short_url,
            long_url: self.long_url,
            created_on: created_on_datetime,
        }
    }
}

#[derive(Serialize,Debug)]
pub struct UrlJson {
    pub id: i32,
    pub short_url: String,
    pub long_url: String,
    pub created_on: DateTime<Utc>,
}