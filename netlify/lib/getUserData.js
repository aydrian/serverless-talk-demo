const fetch = require("node-fetch");
const url = `https://api.github.com/users`;

const getUserData = async function (username) {
  const userData = await fetch(`${url}/${username}`)
    .then((res) => res.json())
    .then((json) => {
      return {
        image: json.avatar_url,
        username: `@${username}`
      };
    })
    .catch((err) => console.error("error:" + err));

  return userData;
};

module.exports = getUserData;
