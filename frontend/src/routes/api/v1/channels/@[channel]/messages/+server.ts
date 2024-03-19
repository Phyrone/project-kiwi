import type {RequestHandler} from "./$types";
import {json} from "@sveltejs/kit";
import {respond_known_errors} from "$lib/RFC9457";
import {r, rethink} from "$lib/server/database";
import {SendMessageRequestSchema} from "$lib/schemas";
import type {SendMessageRequest, SendMessageResponse} from "$lib/request_types";


export const POST: RequestHandler = async ({request, params: {channel}}) => {
    try {
        const request_data_raw = await request.json();

        const request_data: SendMessageRequest = await SendMessageRequestSchema.validateAsync(request_data_raw, {
            cache: true,
            allowUnknown: true
        });
        const result = await r.table("message").insert({
            channel,
            message: request_data.message
        }).run(rethink);


        return json({
            id: result.generated_keys[0]
        } satisfies SendMessageResponse);
    } catch (e) {
        return respond_known_errors(e)
    }
};

export const GET: RequestHandler = async ({params: {channel}}) => {

    const db_query = r.table("message")
        .filter(r.row("channel").eq(channel));

    const cursor = await db_query.run(rethink);

    const messages = [];
    while (cursor.hasNext()) {
        const message = await cursor.next();
        messages.push(message);
    }

    return json({
        messages
    })
}