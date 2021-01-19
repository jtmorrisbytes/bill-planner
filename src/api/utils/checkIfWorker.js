export default function checkIfWorker() {
  if (!self) {
    throw new TypeError(
      "This is a web worker and must be run in a worker context"
    );
  }
}
