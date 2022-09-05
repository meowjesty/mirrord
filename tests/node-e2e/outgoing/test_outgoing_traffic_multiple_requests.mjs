import https from "node:https";
import { setTimeout } from "node:timers";

console.log(">> test_outgoing_traffic_single_request");

function makeRequest(host) {
  const options = {
    hostname: host,
    port: 443,
    path: "/",
    method: "GET",
  };

  const request = https.request(options, (response) => {
    console.log(`>> ${host} statusCode: ${response.statusCode}`);

    response.on("data", (data) => {
      console.log(`>> ${host} response data`);
    });

    response.on("error", (fail) => {
      console.error(`>> ${host} response failed with ${fail}`);
      throw fail;
    });

    response.on("end", () => {
      console.log(`>> ${host} response done `);
    });
  });

  request.on("error", (fail) => {
    console.error(`>> ${host} request failed with ${fail}`);
    throw fail;
  });

  request.on("finish", () => {
    console.log(`>> ${host} request finished`);
  });

  request.end();
}

const hostList = [
  "www.rust-lang.org",
  "www.github.com",
  "www.google.com",
  "www.bing.com",
  "www.yahoo.com",
  "www.baidu.com",
  "www.twitter.com",
  "www.microsoft.com",
  "www.youtube.com",
  "www.live.com",
  "www.msn.com",
  "www.google.com.br",
  "www.yahoo.co.jp",
  "www.qq.com",
  // "www.news.yahoo.co.jp",
];

for (let i = 0; i < 20; i++) {
  setTimeout(() => {
    const host = hostList[i % hostList.length];
    console.log(`>> starting request for ${host}`);
    makeRequest(host);
  }, 0 * i);
}
