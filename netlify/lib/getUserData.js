import fetch from "node-fetch";
const url = `https://api.github.com/users`;

export default async function (username) {
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
}
