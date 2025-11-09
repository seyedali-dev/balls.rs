use crate::MyError;
use actix_web::get;
use log::info;

#[get("/")]
pub async fn index() -> actix_web::Result<&'static str, MyError> {
    let err = MyError { name: "test error" };
    info!("{}", err);
    Err(err)
}
