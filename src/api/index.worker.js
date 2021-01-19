if (!self) {
  throw new TypeError(
    "This file is a web worker and must be loaded in the context of web workers"
  );
}
