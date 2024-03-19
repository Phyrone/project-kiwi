import { j as json } from "../../../../../../../../chunks/index.js";
import { L as LoginRequestUserPassSchema, a as respond_known_errors } from "../../../../../../../../chunks/schemas.js";
import "argon2";
async function tryLoginUserPass(user, password) {
  return void 0;
}
const POST = async ({ request }) => {
  try {
    const request_data_unvalidated = await request.json();
    const request_data = await LoginRequestUserPassSchema.validateAsync(
      request_data_unvalidated,
      {
        abortEarly: false,
        allowUnknown: true,
        cache: true
      }
    );
    const data = tryLoginUserPass(request_data.user, request_data.password);
    return json(
      {
        login: data
      },
      {
        status: 202
      }
    );
  } catch (error) {
    return respond_known_errors(error);
  }
};
export {
  POST
};
