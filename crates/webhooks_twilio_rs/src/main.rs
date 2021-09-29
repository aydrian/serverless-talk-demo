use lamedh_http::{
    http::StatusCode,
    lambda::{lambda, Context, Error},
    IntoResponse, Request, Response,
};
use twilio::twiml::{Message, Twiml};
//use twilio::TwilioError;

#[lambda(http)]
#[tokio::main]
async fn main(_request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    /*let request = request.map(|body| body.as_ref().into());
    let account_sid = std::env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID was not set");
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN was not set");
    let client = twilio::Client::new(&account_sid, &auth_token);

    match client.parse_request::<twilio::Message>(request).await.err() {
        Some(TwilioError::AuthError) => {
            return Ok(Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body(String::from("Signature verification failed."))
                .unwrap());
        }
        Some(TwilioError::BadRequest) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(String::from("Bad Request"))
                .unwrap());
        }
        _ => {}
    }*/

    let mut t = Twiml::new();
    t.add(&Message {
        txt: String::from("Response from rust"),
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/xml")
        .body(t.as_twiml())
        .unwrap())
}
