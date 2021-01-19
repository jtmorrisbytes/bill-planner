// check if fetch is supported on module startup
import "../utils/checkFetch";
import requireJsonResponse from "../utils/requireJsonResponse";
import CONSTANTS from "./constants";

// this module only supports json responses!
export function get(uid = "test") {
  return new Promise((resolve, reject) => {
    fetch(`${CONSTANTS.API_BASE_URL}/transactions?uid=test`)
      .then((response) => {
        if (response.status === 200 && requireJsonResponse(response)) {
          response
            .json()
            .then((parsed) => {
              // if(arra)
              resolve(parsed);
            })
            .catch((e) => {
              console.error("Failed to parse json response", e);
              reject(e);
            });
        } else {
          reject(response);
        }
      })
      .catch(reject);
  });
}

export default { get };
