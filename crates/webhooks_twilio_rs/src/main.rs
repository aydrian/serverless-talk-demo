// use crypto::hmac::Hmac;
// use crypto::mac::{Mac, MacResult};
// use crypto::sha1::Sha1;
use lamedh_http::{
    http::StatusCode,
    lambda::{lambda, Context, Error},
    IntoResponse, Request, Response,
};
// use std::collections::BTreeMap;

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

/*fn verifyTwilioRequest(req: Request) -> bool {
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN was not set");
    let sig = req.headers().get("x-twilio-signature").unwrap().as_bytes();
    let (parts, body) = req.into_parts();
    let body = body.as_ref();

    let post_append = match parts.method {
        Method::POST => {
            let postargs = args_from_urlencoded(&body);
            let append: String = postargs
                .iter()
                .map(|(k, v)| format!("{}{}", k, v))
                .collect();
            append
        }
        _ => return false,
    };

    let effective_uri = format!(
        "https://serverless-talk-demo.netlify.app/webhooks/twilio_rs{}",
        post_append
    );
    let mut hmac = Hmac::new(Sha1::new(), auth_token.as_bytes());
    hmac.input(effective_uri.as_bytes());
    let result = hmac.result();
    let expected = MacResult::new(sig);
    result == expected
}

fn args_from_urlencoded(enc: &[u8]) -> BTreeMap<String, String> {
    url::form_urlencoded::parse(enc).into_owned().collect()
}
*/
