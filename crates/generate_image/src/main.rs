use http::StatusCode;
use image::{DynamicImage, ImageOutputFormat};
use lambda_runtime::{handler_fn, Context, Error};
use og_image_writer::{style, writer::OGImageWriter};
use serde::Deserialize;
use serde_json::{json, Value};

// const WIDTH: u32 = 1280;
// const HEIGHT: u32 = 1280;

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

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    let username = event["queryStringParameters"]["username"].as_str().unwrap();
    let github_user = get_github_user(username).await?;
    let encoded_data = gen_image(github_user).await?;

    Ok(json!({
        "headers": {
            "Content-Type": "image/png",
            "Content-Length": encoded_data.len().to_string()
        },
        "statusCode": StatusCode::OK.as_u16(),
        "body": base64::encode(encoded_data),
        "isBase64Encoded": true
    }))
}

async fn gen_image(github_user: GitHubUser) -> Result<Vec<u8>, Error> {
    let text = format!("Thanks for coming to my talk, {}!", github_user.login);

    let mut writer = OGImageWriter::from_data(
        style::WindowStyle {
            align_items: style::AlignItems::Center,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        },
        include_bytes!("../background.png"),
    ).unwrap();

    /*let mut writer = OGImageWriter::new(style::WindowStyle {
        width: WIDTH,
        height: HEIGHT,
        background_color: Some(style::Rgba([70, 40, 90, 255])),
        align_items: style::AlignItems::Center,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    })
    .unwrap();*/

    let font = Vec::from(include_bytes!("../FiraSans-Bold.ttf") as &[u8]);

    let crl_logo = Vec::from(include_bytes!("../logo.png") as &[u8]);

    let mut logo_container = OGImageWriter::new(style::WindowStyle {
        width: 475,
        height: 67,
        background_color: Some(style::Rgba([223, 246, 245, 0])),
        align_items: style::AlignItems::Start,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    })?;
    logo_container.set_img_with_data(
        &crl_logo,
        475,
        67,
        style::Style {
            margin: style::Margin(0, 0, 0, 0),
            ..Default::default()
        },
    )?;
    writer.set_container(
        &mut logo_container,
        style::Style {
            margin: style::Margin(1165, 0, 0, 60),
            text_align: style::TextAlign::Center,
            position: style::Position::Absolute,
            ..style::Style::default()
        },
    )?;

    writer.set_text(
        &text,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            line_height: 1.8,
            font_size: 100.,
            word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::Start,
            ..style::Style::default()
        },
        font,
    )?;

    /*writer.set_img(
        &github_user.avatar_url,
        280,
        280,
        style::Style {
            margin: style::Margin(0, 0, 34, 0),
            border_radius: style::BorderRadius(100, 100, 100, 100),
            ..style::Style::default()
        },
    )?;*/
    let avatar_data = reqwest::get(github_user.avatar_url).await?.bytes().await?;
    writer.set_img_with_data(
        avatar_data.as_ref(),
        280,
        280,
        style::Style {
            margin: style::Margin(0, 0, 34, 0),
            border_radius: style::BorderRadius(100, 100, 100, 100),
            ..style::Style::default()
        },
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
        /*let github_user: GitHubUser = GitHubUser{
            login: String::from("aydrian"),
            avatar_url: String::from("https://avatars.githubusercontent.com/u/981130?v=4")
        };*/
        let github_user = get_github_user("rainleander").await.unwrap();
        let image = gen_image(github_user).await.unwrap();
        std::fs::write("./test-file.png", image).unwrap()
    }
}