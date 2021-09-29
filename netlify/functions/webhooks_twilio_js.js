import Twilio from "twilio";
import { URLSearchParams } from "url";

const twilioAuthToken = process.env.TWILIO_AUTH_TOKEN;

export async function handler(event, _context) {
  if (event.httpMethod !== "POST") {
    return {
      statusCode: 405,
      headers: { Allow: "POST" },
      body: "Method Not Allowed"
    };
  }

  const twilioSignature = event.headers["x-twilio-signature"];
  const body = Object.fromEntries(new URLSearchParams(event.body));
  console.log(twilioSignature);
  if (
    !Twilio.validateRequest(
      twilioAuthToken,
      twilioSignature,
      "https://serverless-talk-demo.netlify.app/webhooks/twilio",
      body
    )
  ) {
    console.log("Signature verification failed.");
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
