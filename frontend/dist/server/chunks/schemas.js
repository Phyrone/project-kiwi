import { j as json } from "./index.js";
import pg from "postgres";
import r from "rethinkdb";
import Joi from "joi";
const DOCUMENATION_BASE_URL = "http://localhost:3000/docs/";
const DOCUMENTATION_API_BASE_URL = DOCUMENATION_BASE_URL + "developement/api/";
const DOCUMENTATION_API_ERRORS_URL = DOCUMENTATION_API_BASE_URL + "errors/";
function standardizedError(description) {
  return json(
    {
      success: false,
      type: description.type,
      title: description.title,
      detail: description.detail,
      instance: description.instance,
      ...description.additional
    },
    {
      headers: {
        "Content-Type": "application/problem+json",
        Accept: "application/json, application/problem+json",
        "Content-Language": "en"
      },
      status: description.status ?? 500
    }
  );
}
function respond_known_errors(error) {
  if (typeof error !== "object" || error === null) {
    throw error;
  }
  if (error instanceof Error) {
    let description;
    switch (error.name) {
      case "ValidationError":
        description = {
          type: `${DOCUMENTATION_API_ERRORS_URL}common#request-schema`,
          title: "Invalid Schema",
          detail: error.message,
          status: 400,
          additional: {
            // @ts-ignore
            report: error.details
          }
        };
        break;
      case "SyntaxError":
        description = {
          type: `${DOCUMENTATION_API_ERRORS_URL}common#invalid-json`,
          title: "Invalid JSON",
          detail: error.message,
          status: 400
        };
        break;
      default:
        throw error;
    }
    return standardizedError(description);
  } else {
    throw error;
  }
}
pg("postgres://localhost/app2", {
  username: "app2",
  password: "123456",
  debug: (connection, query, parameters) => {
    if (parameters.length > 0)
      console.debug("SQL", "'" + query + "'", "with", parameters);
    else
      console.debug("SQL", "'" + query + "'");
  },
  connection: {
    application_name: "project-kiwi-frontend"
  }
});
async function create_rethink_db() {
  const connection = await r.connect({
    host: "192.168.3.132",
    password: "rethinkdb"
  });
  connection.use("app2");
  return connection;
}
const rethink = await create_rethink_db();
const LoginRequestUserPassSchema = Joi.object({
  user: Joi.string().email({ allowUnicode: true }).required(),
  password: Joi.string().required(),
  remember: Joi.boolean().optional()
});
const SendMessageRequestSchema = Joi.object({
  message: Joi.string().min(1).max(16e3, "utf8").required()
});
const GetRequestSearchParamsSchema = Joi.object({
  since: Joi.string().isoDate().optional(),
  until: Joi.string().isoDate().optional(),
  limit: Joi.number().integer().min(1).optional(),
  offset: Joi.number().integer().min(0).optional(),
  ids: Joi.array().items(Joi.string()).optional(),
  users: Joi.array().items(Joi.string()).optional(),
  watch: Joi.boolean().optional()
});
export {
  GetRequestSearchParamsSchema as G,
  LoginRequestUserPassSchema as L,
  SendMessageRequestSchema as S,
  respond_known_errors as a,
  rethink as r
};
