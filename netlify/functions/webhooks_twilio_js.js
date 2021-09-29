import Twilio from "twilio";

const twilioAuthToken = process.env.TWILIO_AUTH_TOKEN;

export async function handler(event, _context) {
  if (event.httpMethod !== "POST") {
    return {
      statusCode: 405,
      headers: { Allow: "POST" },
      body: "Method Not Allowed"
    };
  }

  const twilioSignature = event.headers["X-Twilio-Signature"];
  if (
    !Twilio.validateRequestWithBody(
      twilioAuthToken,
      twilioSignature,
      "https://serverless-talk-demo.netlify.app/webhooks/twilio",
      event.body
    )
  ) {
    return {
      statusCode: 422,
      body: "Signature verification failed."
    };
  }

  const twiml = new Twilio.twiml.MessagingResponse();
  twiml.message("Response from javascript");

  return {
    statusCode: 200,
    headers: {
      "Content-Type": "text/xml"
    },
    body: twiml.toString()
  };
}
