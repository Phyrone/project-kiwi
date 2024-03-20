import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';

export const PUT: RequestHandler = async () => {
	return json({});
};
