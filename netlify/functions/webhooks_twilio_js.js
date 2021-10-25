const Twilio = require("twilio");
const { Client } = require("pg");
const format = require("pg-format");
const fs = require("fs");
const path = require("path");

const { withVerifyTwilio } = require("../lib/withVerifyTwilio");

const client = new Client({
  ssl: {
    ca: fs.readFileSync(path.join(__dirname, "../certs/cc-ca.crt")).toString()
  }
});

async function twilioHandler(event, _context) {
  const { parsedBody } = event;
  console.log(parsedBody);

  const { Body, FromCity, FromState, FromZip, FromCountry } = parsedBody;
  const username = Body.trim();

  try {
    await client.connect();
    const sqlStatement = format(
      "INSERT INTO messages (github_username, sms_location, function_used) VALUES (%L, %L, 'JavaScript');",
      username,
      { FromCity, FromState, FromZip, FromCountry }
    );
    await client.query(sqlStatement);
  } catch (ex) {
    console.log("Error inserting message", ex);
  } finally {
    await client.end();
  }

  const twiml = new Twilio.twiml.MessagingResponse();
  const message = twiml.message();
  message.body("JavaScript Function Response");
  message.media(
    `https://serverless-talk-demo.netlify.app/generate/image?username=${username}`
  );

  return {
    statusCode: 200,
    headers: {
      "Content-Type": "text/xml"
    },
    body: twiml.toString()
  };
}

exports.handler = withVerifyTwilio(twilioHandler);
