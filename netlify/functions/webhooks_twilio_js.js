const Twilio = require("twilio");
const playwright = require("playwright-aws-lambda");
const fs = require("fs");
const path = require("path");

const cloudinary = require("cloudinary").v2;
cloudinary.config({
  cloud_name: process.env.CLOUDINARY_CLOUD_NAME,
  api_key: process.env.CLOUDINARY_API_KEY,
  api_secret: process.env.CLOUDINARY_API_SECRET
});

const { withVerifyTwilio } = require("../lib/withVerifyTwilio");
const { getUserData } = require("../lib/getUserData");

const script = fs.readFileSync(
  path.resolve(__dirname, "./shout-out-image/image.js"),
  "utf-8"
);

async function twilioHandler(event, _context) {
  const { parsedBody } = event;
  console.log(parsedBody);

  const userId = parsedBody.Body.trim();
  const userData = await getUserData(userId);
  console.log(userData);

  const browser = await playwright.launchChromium();
  const context = await browser._defaultContext;
  const page = await context.newPage();

  page.setViewportSize({
    width: 1200,
    height: 630
  });

  await page.setContent(`<!DOCTYPE html>
  <html>
    <head>
      <meta charset="utf-8" />
      <link
        href="https://fonts.googleapis.com/css?family=Nunito+Sans:400,700,800,900&display=swap"
        rel="stylesheet"
      />
      <link rel="preload" href="${
        userData ? userData.image : ""
      }" as="image" media="(max-width: 600px)" />
    </head>
    <body>
      <div id="app">
        <div
          style="
            display: flex;
            align-items: center;
            text-align: center;
            font-size: 72px;
            font-weight: 900;
            line-height: 96px;
            font-family: 'Nunito Sans', 'Helvetica Neue', Helvetica, Arial,
              sans-serif;
            width: 1200px;
            height: 630px;
            overflow: hidden;
          "
        >
          Thanks for coming to my talk!
        </div>
      </div>
    </body>
  </html>
  `);

  if (userData) {
    await page.addScriptTag({
      content: `
      window.image = "${userData.image}";
      window.username = "${userData.username}";
    `
    });
    await page.addScriptTag({ content: script });
  }

  const boundingRect = await page.evaluate(() => {
    const app = document.getElementById("app");
    const { x, y, width, height } = app.children[0].getBoundingClientRect();
    return { x, y, width, height };
  });

  const screenshotBuffer = await page.screenshot({ clip: boundingRect });
  await browser.close();

  const cloudinaryResult = await cloudinaryPromise(screenshotBuffer, {
    public_id: `serverless-talk/${userId}`
  });

  const twiml = new Twilio.twiml.MessagingResponse();
  const message = twiml.message();
  message.body("Response from javascript");
  message.media(cloudinaryResult.secure_url);

  return {
    statusCode: 200,
    headers: {
      "Content-Type": "text/xml"
    },
    body: twiml.toString()
  };
}

function cloudinaryPromise(shotResult, cloudinary_options) {
  return new Promise(function (res, rej) {
    cloudinary.uploader
      .upload_stream(cloudinary_options, function (error, cloudinary_result) {
        if (error) {
          console.error("Upload to cloudinary failed: ", error);
          rej(error);
        }
        res(cloudinary_result);
      })
      .end(shotResult);
  });
}

exports.handler = withVerifyTwilio(twilioHandler);
