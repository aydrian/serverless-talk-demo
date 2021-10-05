use crypto::hmac::Hmac;
use crypto::mac::{Mac, MacResult};
use crypto::sha1::Sha1;
use http::StatusCode;
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    let (verified, parsed_body) = parse_twilio_event(event);
    if !verified {
        println!("Signature verification failed.");
        return Ok(json!({
            "statusCode": StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            "body": "Signature verification failed."
        }));
    }

    println!("parsed body: {:?}", parsed_body);
    let username = parsed_body.get("Body").unwrap().trim();
    println!("username: {}", username);
    let media_url = format!(
        "https://serverless-talk-demo.netlify.app/.netlify/functions/generate_image?username={}",
        username
    );

    let twiml = format!(
        "<Response><Message><Body>Hello {} Response from rust</Body><Media>{}</Media></Message></Response>",
        username, media_url
    );

    Ok(json!({
        "statusCode": StatusCode::OK.as_u16(),
        "headers": {
            "Content-Type": "text/xml",
        },
        "body": twiml
    }))
}

fn parse_twilio_event(event: Value) -> (bool, BTreeMap<String, String>) {
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN was not set");
    let twilio_signature = event["headers"]["X-Twilio-Signature"].as_str().unwrap();
    println!("twilio signature: {}", twilio_signature);
    let post_args: BTreeMap<String, String> =
        url::form_urlencoded::parse(event["body"].as_str().unwrap().as_bytes())
            .into_owned()
            .collect();
    let append: String = post_args
        .iter()
        .map(|(k, v)| format!("{}{}", k, v))
        .collect();
    let effective_uri = format!(
        "https://https://serverless-talk-demo.netlify.app/.netlify/functions/webhooks_twilio_rs{}",
        append
    );
    let mut hmac = Hmac::new(Sha1::new(), auth_token.as_bytes());
    hmac.input(effective_uri.as_bytes());
    let result = hmac.result();
    let expected = MacResult::new(twilio_signature.as_bytes());
    (result == expected, post_args)
}
