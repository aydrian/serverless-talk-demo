use lamedh_http::{
    http::StatusCode,
    lambda::{lambda, Context, Error},
    IntoResponse, Request, Response,
};

#[lambda(http)]
#[tokio::main]
async fn main(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(String::from("OK"))
        .unwrap())
}
