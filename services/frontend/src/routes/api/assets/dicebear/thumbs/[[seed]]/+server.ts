import type { RequestHandler } from './$types';
import * as style from '@dicebear/thumbs';
import { createAvatar } from '@dicebear/core';
import generateEtag from 'etag';

export const trailingSlash = 'ignore';

export const GET: RequestHandler = async ({ params: { seed } }) => {
	const avatar = createAvatar(style, {
		seed
	});
	const body = await avatar.toArrayBuffer();
	const etag = generateEtag(Buffer.from(body), {
		weak: false
	});

	return new Response(body, {
		headers: {
			'Content-Type': 'image/svg+xml',
			'ETag': etag,
			'Cache-Control': 'public, max-age=31536000, immutable'
		}
	});
};