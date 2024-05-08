import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';


export const GET: RequestHandler = async ({ request: { headers } }) => {
		const user_lang = headers.get('accept-language')?.split(',');
		return json(user_lang);
	}
;