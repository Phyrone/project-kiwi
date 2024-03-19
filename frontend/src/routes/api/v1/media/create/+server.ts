import type {RequestHandler} from "./$types"
import {json} from "@sveltejs/kit";

export const POST: RequestHandler = async ({fetch}) => {

    return json({})
}