use rocket::tokio;
use url_shortener;

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let _ = url_shortener::rocket().launch().await;
    });
}