use lamedh_http::{
    http::StatusCode,
    lambda::{lambda, Context, Error},
    IntoResponse, Request, Response,
};

#[lambda(http)]
#[tokio::main]
async fn main(_request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // TODO: Verify signature
    let twiml = format!(
        "<Response><Message><Body>{}</Body></Message></Response>",
        "Response from rust"
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/xml")
        .body(twiml)
        .unwrap())
}
