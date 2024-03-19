import { j as json } from "../../../../../../chunks/index.js";
const GET = async ({ params }) => {
  let { id } = params;
  return json({
    commming: "soon",
    params: {
      id
    }
  });
};
export {
  GET
};
