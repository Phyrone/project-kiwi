import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ fetch, request }) => {
	let request_data = await request.json();

	return json({});
};
