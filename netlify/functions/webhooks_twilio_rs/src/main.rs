use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use crypto::hmac::Hmac;
use crypto::mac::{Mac, MacResult};
use crypto::sha1::Sha1;
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    event: ApiGatewayProxyRequest,
    _: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (verified, parsed_body) = parse_twilio_event(event);
    if !verified {
        println!("Signature verification failed.");
        return Ok(ApiGatewayProxyResponse {
            status_code: 422,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(String::from("Signature verification failed."))),
            is_base64_encoded: Some(false),
        });
    }

    println!("parsed body: {:?}", parsed_body);
    let username = parsed_body.get("Body").unwrap().trim();
    println!("username: {}", username);
    let media_url = format!(
        "https://serverless-talk-demo.netlify.app/generate/image?username={}",
        username
    );

    let twiml = format!(
        "<Response><Message><Body>Hello {}\nRust Function Response</Body><Media>{}</Media></Message></Response>",
        username, media_url
    );

    let mut headers = HeaderMap::new();
    headers.insert(http::header::CONTENT_TYPE, "text/xml".parse().unwrap());

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(twiml)),
        is_base64_encoded: Some(false),
    })
}

fn parse_twilio_event(event: ApiGatewayProxyRequest) -> (bool, BTreeMap<String, String>) {
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN was not set");
    let twilio_signature = event
        .headers
        .get("x-twilio-signature")
        .unwrap()
        .to_str()
        .unwrap();
    println!("twilio signature: {}", twilio_signature);
    let post_args: BTreeMap<String, String> =
        url::form_urlencoded::parse(event.body.unwrap().as_bytes())
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
