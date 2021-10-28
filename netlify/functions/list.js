const { PrismaClient } = require("@prisma/client");
const prisma = new PrismaClient();

exports.handler = async function handler(_event, _context) {
  const messages = await prisma.message.findMany();
  return {
    statusCode: 200,
    body: JSON.stringify({ messages })
  };
};
