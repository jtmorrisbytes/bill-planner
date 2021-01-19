import checkIfWorker from "../utils/checkIfWorker";
import CONSTANTS from "./constants";
import network from "./network";
checkIfWorker();
// self.postMessage()
/*eslint-disable */
self.onmessage = function processMessage(event, data) {
  self.console.log("client ", event, data);
  if (!Array.isArray(event.data) && typeof event.data === "object") {
    const { data } = event;
    switch (data.type) {
      case CONSTANTS.GET_TRANSACTIONS:
        self.postMessage({ type: "GET_TRANSACTIONS_PENDING", payload: null });
        network.get().then((data) => {
          self
            .postMessage({
              type: "GET_TRANSACTIONS_RESOLVED",
              payload: data,
            })
            .catch((e) => {
              self.postMessage({
                type: "GET_TRANSACTIONS_FAILED",
                payload: e,
              });
            });
        });
        break;

      default:
        self.postMessage({
          type: CONSTANTS.ERROR,
          payload: `Invalid Message: ${String(data.type)}`,
        });
        break;
    }
  } else {
    self.postMessage({
      type: CONSTANTS.ERROR,
      payload: "Message must be an object",
    });
    throw new TypeError("Message Must be an object ");
  }
};
/*eslint-enable */
