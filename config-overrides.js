const { default: WasmPackPlugin } = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

module.exports = function override(config, env) {
  const wasmExtensionRegExp = /\.wasm$/;

  // config.resolve.extensions.push(".wasm");

  config.module.rules.forEach((rule) => {
    (rule.oneOf || []).forEach((oneOf) => {
      if (oneOf.loader && oneOf.loader.indexOf("file-loader") >= 0) {
        // make file-loader ignore WASM files
        oneOf.exclude.push(wasmExtensionRegExp);
      }
    });
  });

  // add a dedicated loader for WASM
  // config.experiments = { asyncWebAssembly: true };
  // config.module.rules.push({
  //   test: wasmExtensionRegExp,
  //   include: [path.resolve(__dirname, "src/shared/")],
  //   type: "javascript/auto",
  //   use: [
  //     {
  //       loader: require.resolve("raw-loader"),
  //       // options: { export: "instance" },
  //     },
  //   ],
  // });
  config.module.rules.push({
    test: wasmExtensionRegExp,
    type: "webassembly/experimental",
  });
  // config.plugins(
  //   new WasmPackPlugin({
  //     crateDirectory: path.resolve(__dirname, "src/shared/decimalS"),
  //   })
  // );
  // config.experiments = {
  //   asyncWebAssembly: true,
  // };
  return config;
};
