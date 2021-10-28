use crypto::hmac::Hmac;
use crypto::mac::{Mac, MacResult};
use crypto::sha1::Sha1;
use http::StatusCode;
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};
use sqlx::postgres::PgPool;
use std::collections::BTreeMap;
use std::env;

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
    let from_city = "";
    let from_state = "";
    let from_zip = "";
    let from_country = "";

    let location = json!({
        "FromCity":from_city, "FromState": from_state, "FromZip": from_zip, "FromCountry": from_country
    });

    if let Ok(ret) = add_message(username, location).await {
        println!("Message added: {}", ret);
    }

    let media_url = format!(
        "https://serverless-talk-demo.netlify.app/generate/image?username={}",
        username
    );

    let twiml = format!(
        "<Response><Message><Body>Hello {}\nRust Function Response</Body><Media>{}</Media></Message></Response>",
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
    let twilio_signature = event["headers"]["x-twilio-signature"].as_str().unwrap();
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
        "https://serverless-talk-demo.netlify.app/webhooks/twilio{}",
        append
    );
    println!("effective uri: {}", effective_uri);
    let mut hmac = Hmac::new(Sha1::new(), auth_token.as_bytes());
    hmac.input(effective_uri.as_bytes());
    let result = hmac.result();
    let expected = MacResult::new(&base64::decode(twilio_signature.as_bytes()).unwrap());
    (result == expected, post_args)
}

async fn add_message(username: &str, location: Value) -> Result<String, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL was not set.");
    dbg!(&database_url);

    let pool = PgPool::connect(&database_url).await?;

    let rec = sqlx::query!(
        r#"
        INSERT INTO messages (github_username, sms_location, function_used) 
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        username,
        location,
        "Rust"
    )
    .fetch_one(&pool)
    .await?;
    dbg!(&rec);

    Ok(rec.id.to_string())
}
