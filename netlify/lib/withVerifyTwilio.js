const { validateRequest } = require("twilio");
const { URLSearchParams } = require("url");

const twilioAuthToken = process.env.TWILIO_AUTH_TOKEN;

exports.withVerifyTwilio = (handler) => {
  return async (event, context) => {
    if (event.httpMethod !== "POST") {
      return {
        statusCode: 405,
        headers: { Allow: "POST" },
        body: "Method Not Allowed"
      };
    }

    const twilioSignature = event.headers["x-twilio-signature"];
    const parsedBody = Object.fromEntries(new URLSearchParams(event.body));

    if (
      !validateRequest(
        twilioAuthToken,
        twilioSignature,
        "https://serverless-talk-demo.netlify.app/webhooks/twilio",
        parsedBody
      )
    ) {
      console.log("Signature verification failed.");
      return {
        statusCode: 422,
        body: "Signature verification failed."
      };
    }

    return handler({ ...event, parsedBody }, context);
  };
};
