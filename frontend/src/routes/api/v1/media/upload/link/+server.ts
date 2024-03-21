import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { minio } from '$lib/server/minio';
import ms from 'ms';
import type { UploadLinkRequest } from '$lib/request_types';
import { standardizedError } from '$lib/RFC9457';


export const POST: RequestHandler = async ({ request }) => {
	const { file_name, ttl: ttl_raw }: UploadLinkRequest = await request.json();
	let ttl: number;

	if (ttl_raw && typeof ttl_raw === 'number') {
		ttl = ttl_raw;
	} else if (ttl_raw && typeof ttl_raw === 'string') {
		ttl = ms(ttl_raw);
	} else if (ttl_raw) {
		//TODO add error type
		return standardizedError({
			type: 'TODO',
			title: 'Invalid TTL',
			detail: 'The specified \'ttl\' field in the request ist not a string or a number or undefined',
			status: 400
		});
	} else {
		ttl = ms('10m');
	}

	const link: string = await new Promise((resolve, reject) => {
		minio.presignedPutObject('app2', file_name, ttl, (error, result) => {
			if (error) {
				reject(error);
			} else {
				resolve(result);
			}
		});
	});
	return json({
		link
	});
};