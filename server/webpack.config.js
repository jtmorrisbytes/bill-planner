const path = require("path");
const webpack = require("webpack");

const NodemonPlugin = require("nodemon-webpack-plugin");
const DotenvPlugin = require("dotenv-webpack");

const package = require("./package.json");

let nodemonConfig = {};
try {
  nodemonConfig = require("./nodemon.json");
} catch {
  console.log("nodemon.json not found. nodemon will use defaults");
}
// automatically generate externals based on package.dependancies
let packageExternals = Object.keys(package.dependencies);

function requireExt(s = "", path) {
  return { [s]: path ? `require('${path}')` : `require('${s}')` };
}

module.exports = {
  target: "async-node",
  externals: {
    "pg-native": "pg-native",
    // ...packageExternals.reduce((extern) => {
    //   return { [extern]: `require(\\"${extern}\\")` };
    // }, {}),
    debug: "require('debug')",
    ...requireExt("massive"),
  },
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: "babel-loader",
          options: {
            presets: [
              [
                "@babel/preset-env",
                { targets: { node: true }, modules: "commonjs" },
              ],
            ],
            // plugins: ['@babel/plugin-proposal-object-rest-spread']
          },
        },
      },
    ],
  },
  output: {
    filename: "index.js",
  },

  plugins: [
    new NodemonPlugin(nodemonConfig),
    new DotenvPlugin({ path: "./.env", systemvars: true, silent: false }),
    new webpack.ProvidePlugin({
      dotenv: "dotenv",
    }),
  ],
};
