use crate::{database::{self, Db}};
use crate::errors::{Errors, FieldValidator};
use rocket::serde::json::{json, Value, Json};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct NewUrl {
    url: NewUrlData,
}

#[derive(Deserialize, Validate)]
pub struct NewUrlData {
    #[validate(length(min = 1))]
    long_url: String
}

// fn base62_encode(mut num: u64) -> String {
//     let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
//     let mut string = String::new();
//     while num > 0 {
//         let remainder = (num % 62) as usize;
//         num /= 62;
//         string = format!("{}{}", chars.chars().nth(remainder).unwrap(), string);
//     }
//     string
// }


#[post("/url/shorten", format = "application/json", data = "<payload>")]
pub async fn post_url(
    payload: Json<NewUrl>,
    db: Db,
) -> Result<Value, Errors> {

    let url = payload.into_inner().url;
    let mut extractor = FieldValidator::validate(&url);
    let long_url = extractor.extract("short_url", Some(url.long_url));
    extractor.check()?;

    if db.run(move |conn| database::url::find_by_long_url(conn, &long_url)).await {
        return Ok(json!({ "url": "redirecting" }));
    }

    let result = db
    .run(move |conn| {
        database::url::get_next_id(conn)
    })
    .await;

    match result {
        Err(_error) => {
            Err(Errors::InternalServerError(String::from("Hello, world!")))
        }
        Ok(id) => {
            Ok(json!({ "url": (id + 1).to_string()}))
        }
    }
    
}