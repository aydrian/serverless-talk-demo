const Twilio = require("twilio");

const { withVerifyTwilio } = require("../lib/withVerifyTwilio");

async function twilioHandler(event, _context) {
  const { parsedBody } = event;
  console.log(parsedBody);

  const username = parsedBody.Body.trim();

  const twiml = new Twilio.twiml.MessagingResponse();
  const message = twiml.message();
  message.body("Response from javascript");
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
