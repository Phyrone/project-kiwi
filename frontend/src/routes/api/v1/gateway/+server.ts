import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import type { GatewayDataResponse } from '$lib/request_types';

export const GET: RequestHandler = async ({ locals }) => {
	locals.session;

	return json({
		link: '//localhost:7080'
	} satisfies GatewayDataResponse);
};