// use crypto::hmac::Hmac;
// use crypto::mac::{Mac, MacResult};
// use crypto::sha1::Sha1;
use http::StatusCode;
use lambda_runtime::{handler_fn, Context, Error};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Event {
    body: String,
}

#[derive(Deserialize)]
struct GitHubUser {
    login: String,
    //id: u32,
    //name: String,
    avatar_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(event: Event, _: Context) -> Result<Value, Error> {
    let parsed_body: BTreeMap<String, String> = url::form_urlencoded::parse(event.body.as_bytes())
        .into_owned()
        .collect();
    println!("{:?}", parsed_body);
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://api.github.com/users/{}", "aydrian"))
        .header(reqwest::header::USER_AGENT, "rust-reqwest")
        .send()
        .await
        .unwrap()
        .json::<GitHubUser>()
        .await
        .unwrap();
    println!("{}: {}", &res.login, &res.avatar_url);

    // TODO: Verify signature
    let twiml = format!(
        "<Response><Message><Body>Hello {} Response from rust</Body><Media>{}</Media></Message></Response>",
        res.login, res.avatar_url
    );

    Ok(json!({
        "statusCode": StatusCode::OK.as_u16(),
        "headers": {
            "Content-Type": "text/xml",
        },
        "body": twiml
    }))
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
