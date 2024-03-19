import { j as json } from "../../../../../../chunks/index.js";
const GET = async ({ params }) => {
  let { id, domain } = params;
  return json({
    commming: "soon",
    params: {
      id,
      domain
    }
  });
};
export {
  GET
};
