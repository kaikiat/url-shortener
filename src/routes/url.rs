use crate::{database::{self, Db}};
use crate::errors::{Errors, FieldValidator};
use rocket::{serde::json::{json, Value, Json}, response::Redirect};
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

fn base62_encode(mut num: u64) -> String {
    let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut string = String::new();
    while num > 0 {
        let remainder = (num % 62) as usize;
        num /= 62;
        string = format!("{}{}", chars.chars().nth(remainder).unwrap(), string);
    }
    string
}


#[post("/url/shorten", format = "application/json", data = "<payload>")]
pub async fn post_url(
    payload: Json<NewUrl>,
    db: Db,
) -> Result<Value, Errors> {

    let url = payload.into_inner().url;
    let mut extractor = FieldValidator::validate(&url);
    let long_url = extractor.extract("short_url", Some(url.long_url));
    extractor.check()?;

    let long_url_clone = long_url.clone();

    if db.run(move |conn| database::url::find_by_long_url(conn, &long_url_clone)).await {
        return Err(Errors::InternalServerError(String::from("url already exists")));
    }

    let result = db
    .run(move |conn| {
        database::url::get_next_id(conn)
    })
    .await;

    match result {
        Err(_error) => {
            Err(Errors::InternalServerError(String::from("an error occurred while shortening the url")))
        }
        Ok(id) => {
            let short_url = base62_encode((id + 1) as u64);
            let url = db
            .run(move |conn| {
                database::url::create(
                    conn,
                    &short_url,
                    &long_url,
                )
            })
            .await;
            Ok(json!({ "url": url }))
        }
    }
    
}

#[get("/<short_url>")]
pub async fn get_url(short_url: &str, db: Db) -> Result<Redirect, Errors> {
    let short_url_clone = short_url.to_string(); 
    let url = db
        .run(move |conn| database::url::find(conn, &short_url_clone))
        .await;
    match url {
        Ok(url) =>  Ok(Redirect::to(url)),
        Err(_) =>  Err(Errors::InternalServerError(String::from("an error occurred while getting the short_url"))),
    }
}

