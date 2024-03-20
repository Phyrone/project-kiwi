import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ params }) => {
	let { id, domain } = params;

	return json({
		commming: 'soon',
		params: {
			id,
			domain
		}
	});
};
