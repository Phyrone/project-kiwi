import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { respond_known_errors } from '$lib/RFC9457';
import { r, rethink } from '$lib/server/database';
import { GetRequestSearchParamsSchema, SendMessageRequestSchema } from '$lib/schemas';
import type {
	GetRequestSearchParams,
	SendMessageRequest,
	SendMessageResponse
} from '$lib/request_types';
import qs from 'qs';
import type { ValidationResult } from 'joi';
import { todo } from '$lib/utils';
import { sse } from '$lib/sse';

export const PUT: RequestHandler = async ({ request, params: { channel } }) => {
	try {
		const request_data_raw = await request.json();

		const request_data: SendMessageRequest = await SendMessageRequestSchema.validateAsync(
			request_data_raw,
			{
				cache: true,
				allowUnknown: true
			}
		);

		const result = await r
			.table('message')
			.insert({
				channel,
				//reply_to: request_data.reply_to,
				message: request_data.message,
				timestamp: r.now()
			})
			.run(rethink);

		return json({
			id: result.generated_keys[0]
		} satisfies SendMessageResponse);
	} catch (e) {
		return respond_known_errors(e);
	}
};

export const GET: RequestHandler = async ({ url, request, params: { channel } }) => {
	const params_raw = qs.parse(url.search, { ignoreQueryPrefix: true });
	const { error, value: search } = GetRequestSearchParamsSchema.validate(params_raw, {
		cache: true,
		allowUnknown: true,
		stripUnknown: false
	}) as ValidationResult<GetRequestSearchParams>;
	if (error) {
		todo('invalid query params error handling');
	}

	const channel_messages = r.table('message').filter(r.row('channel').eq(channel));
	let query = channel_messages;
	if (search.limit) {
		query = channel_messages.limit(search.limit);
	}
	if (search.offset) {
		query = channel_messages.skip(search.offset);
	}

	if (search.watch) {
		const cursor = await query
			.changes({
				includeInitial: true,
				squash: true,
				changefeedQueueSize: 100000,
				includeOffsets: false,
				includeStates: true,
				includeTypes: true
			})
			.run(rethink);

		return sse(async (stream) => {
			try {
				while (!stream.isClosed) {
					const message = await cursor.next();
					stream.send({
						event: 'message',
						data: message
					});
				}
			} finally {
				await cursor.close();
			}
		});
	} else {
		query = query.withFields('id', 'message', 'timestamp').orderBy(r.desc('timestamp'));

		const cursor = await query.run(rethink);

		return json({
			search,
			search_raw: url.search,
			messages: await cursor.toArray()
		});
	}
};
