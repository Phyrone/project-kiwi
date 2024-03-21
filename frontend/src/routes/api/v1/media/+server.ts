import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { minio } from '$lib/server/minio';
import ms from 'ms';

export const POST: RequestHandler = async () => {


	const link = await new Promise((resolve, reject) => {
		minio.presignedPutObject('app2', 'test.json', ms('10m'), (error, result) => {
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
