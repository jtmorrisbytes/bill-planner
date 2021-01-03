const express = require("express");
module.exports = function (req, res, next) {
  if (req.is("json")) {
    next();
  } else {
    res.status(415).send("Expecting JSON body");
  }
};
