const Twilio = require("twilio");
const { PrismaClient } = require("@prisma/client");

const { withVerifyTwilio } = require("../lib/withVerifyTwilio");
const prisma = new PrismaClient();

async function twilioHandler(event, _context) {
  const { parsedBody } = event;
  console.log(parsedBody);

  const { Body, FromCity, FromState, FromZip, FromCountry } = parsedBody;
  const username = Body.trim();

  try {
    await prisma.message.create({
      data: {
        github_username: username,
        sms_location: { FromCity, FromState, FromZip, FromCountry },
        function_used: "JavaScript"
      }
    });
  } catch (ex) {
    console.log("Error inserting message", ex);
  } finally {
    await prisma.$disconnect();
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
