if (!fetch) {
  throw new Error(
    "This module relies on the Fetch API, but fetch() was not found in the global scope"
  );
}
