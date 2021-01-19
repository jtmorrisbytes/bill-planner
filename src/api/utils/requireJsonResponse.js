// this function only works with fetch!!
export default function requireJsonResponse(response, reject) {
  try {
    return response.headers.get("Content-Type").includes("application/json");
  } catch (e) {
    if (process?.env?.NODE_ENV === "development") {
      console.error(e);
    }
    return false;
  }
}
