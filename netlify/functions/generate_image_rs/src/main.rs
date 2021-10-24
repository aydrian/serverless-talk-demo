use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use image::{DynamicImage, ImageOutputFormat};
use lambda_runtime::{handler_fn, Context, Error};
use og_image_writer::{style, writer::OGImageWriter};
use rand::seq::SliceRandom;
use serde::Deserialize;

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

async fn handler(
    event: ApiGatewayProxyRequest,
    _: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let username = event
        .query_string_parameters
        .get("username")
        .unwrap()
        .as_str();
    let github_user = get_github_user(username).await?;
    let encoded_data = gen_image(github_user).await?;

    let mut headers = HeaderMap::new();
    headers.insert(http::header::CONTENT_TYPE, "image/png".parse().unwrap());
    headers.insert(
        http::header::CONTENT_LENGTH,
        encoded_data.len().to_string().parse().unwrap(),
    );

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(base64::encode(encoded_data))),
        is_base64_encoded: Some(true),
    })
}

async fn gen_image(github_user: GitHubUser) -> Result<Vec<u8>, Error> {
    let mut writer = OGImageWriter::from_data(
        style::WindowStyle {
            align_items: style::AlignItems::Center,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        },
        include_bytes!("../background.png"),
    )
    .unwrap();

    let font = Vec::from(include_bytes!("../FiraSans-Bold.ttf") as &[u8]);

    let border_radius = 375 / 2 as u32;

    let avatar_data = reqwest::get(github_user.avatar_url).await?.bytes().await?;
    writer.set_img_with_data(
        avatar_data.as_ref(),
        375,
        375,
        style::Style {
            border_radius: style::BorderRadius(
                border_radius,
                border_radius,
                border_radius,
                border_radius,
            ),
            position: style::Position::Absolute,
            top: Some(450),
            left: Some(452),
            ..style::Style::default()
        },
    )?;

    let phrases = vec!["Does this work", "Is this right", "Was this fast"];
    let text = format!(
        "{}, @{}?",
        phrases
            .choose(&mut rand::thread_rng())
            .unwrap_or(&phrases[0]),
        github_user.login
    );

    writer.set_text(
        &text,
        style::Style {
            position: style::Position::Absolute,
            top: Some(876),
            line_height: 2.5,
            font_size: 96.,
            word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::Center,
            ..style::Style::default()
        },
        font,
    )?;

    writer.paint()?;

    let img = writer.image()?;
    let mut buf = vec![];
    let dyn_img = DynamicImage::ImageRgba8(img);
    dyn_img.write_to(&mut buf, ImageOutputFormat::Png).unwrap();

    Ok(buf)
}

async fn get_github_user(username: &str) -> Result<GitHubUser, Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://api.github.com/users/{}", username))
        .header(reqwest::header::USER_AGENT, "rust-reqwest")
        .send()
        .await?
        .json::<GitHubUser>()
        .await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn generates_image() {
        let github_user = get_github_user("ChristopherBiscardi").await.unwrap();
        let image = gen_image(github_user).await.unwrap();
        std::fs::write("./test-file.png", image).unwrap()
    }
}
