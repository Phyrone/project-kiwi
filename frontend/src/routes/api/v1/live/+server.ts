import type { RequestHandler } from './$types';
import type { LiveSearchParams } from '$lib/request_types';
import qs from 'qs';
import { LiveView } from './live';
import { sse } from '$lib/sse';
import { LiveSearchParamsSchema } from '$lib/schemas';

export const GET: RequestHandler = async ({ url }) => {
	const params: LiveSearchParams = qs.parse(url.search, {
		ignoreQueryPrefix: true,
		charset: 'utf-8'
	}) as LiveSearchParams;
	LiveSearchParamsSchema.validate(params, {
		allowUnknown: true,
		abortEarly: false,
		stripUnknown: true
	});

	return sse(async (session) => {
		const live_view = new LiveView(params.intents ?? [], (event) => {
			session.send(event);
		});
		session.onClose = () => {
			live_view.stop();
		};
		await live_view.start();
	});
};
