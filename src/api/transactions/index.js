// this file is a wrapper around the web worker
import SharedWorker from "./index.worker.js";
import CONSTANTS from "./constants";
const worker = SharedWorker();
console.log(worker);

worker.onMessage = function processWorkerResponse(message, data) {
  console.log("worker response", message, data);
};
worker.onError = console.error;
// worker.postMessage({ type: "INIT", payload: "hello world" });
export function getTransactions(onRequestStart) {
  return new Promise(async (resolve, reject) => {
    function onMessage({ data }) {
      switch (data.type) {
        case CONSTANTS.ERROR:
          console.error(`Worker Error: ${data.payload}`);
          reject(data);
          break;
        case CONSTANTS.GET_TRANSACTIONS_PENDING:
          onRequestStart?.();
          break;
        default:
          reject(data);
          console.error("client recieved invalid message type from worker");
      }
      resolve(data);
    }
    function onError(error) {
      worker.removeEventListener("message", onMessage);
      worker.removeEventListener("messageerror", onError);
      reject(error);
    }
    worker.addEventListener("message", onMessage);
    worker.addEventListener("messageerror", onError);
    worker.postMessage({ type: CONSTANTS.GET_TRANSACTIONS, payload: null });
  });
}
export default { getTransactions };
