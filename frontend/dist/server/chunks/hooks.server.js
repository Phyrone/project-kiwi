import "./index.js";
const handle = async ({ event, resolve }) => {
  return resolve(event);
};
const handleError = async ({ status, message, error, event }) => {
  const response = errorMessageByCode(status, message, event.url.pathname);
  console.log("response", response);
  if (response) {
    console.error("unhandled error", error);
    event.setHeaders({
      "Content-Type": "application/problem+json",
      "Accept": "application/json, application/problem+json",
      "Content-Language": "en"
    });
    return response;
  } else {
    return;
  }
};
function errorMessageByCode(code, message, path) {
  let part;
  switch (true) {
    case code >= 500:
      part = {
        type: "https://www.rfc-editor.org/rfc/rfc9110.html#name-500-internal-server-error",
        title: "Internal Server Error"
      };
      break;
    case code == 400:
      part = {
        type: "https://www.rfc-editor.org/rfc/rfc9110.html#name-400-bad-request",
        title: "Bad Request"
      };
      break;
    default:
      return void 0;
  }
  return {
    success: false,
    detail: message,
    instance: path,
    ...part
  };
}
export {
  handle,
  handleError
};
