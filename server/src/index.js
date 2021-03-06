const CONFIG = require("dotenv").config().parsed || {};
const express = require("express");
const massive = require("massive");
const hello = require("./hello").hello;
const api = express();
const requireJSON = require("./guards/requireJSON");

// when using webpack, environment variables must be
// referenced using process.env.*
const NODE_ENV = CONFIG.NODE_ENV || process.env.NODE_ENV;
const DATABASE_URL = CONFIG.DATABASE_URL || process.env.DATABASE_URL;

const HOST = process.env.HOST || "0.0.0.0";
const PORT = process.env.PORT || 8000;
// this api communicates using json
api.use(express.json());
// require a json body for all requests that need a body
api.patch("*", requireJSON);
api.put("*", requireJSON);
api.post("*", requireJSON);

massive({
  connectionString: DATABASE_URL,
  ssl: {
    rejectUnauthorized: false,
  },
})
  .then((db) => {
    api.set("db", db);
    db.test().then(console.log);
    api.listen(PORT, HOST, () => {
      console.log(`Listening on http://${HOST}:${PORT}`);
    });
  })
  .catch((e) => {
    console.error(e);
    console.dir(CONFIG);
  });
