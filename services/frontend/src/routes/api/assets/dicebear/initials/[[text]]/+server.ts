import type { RequestHandler } from './$types';
import generateEtag from 'etag';
import * as style from '@dicebear/initials';
import { createAvatar } from '@dicebear/core';

export const trailingSlash = 'ignore';

export const GET: RequestHandler = async ({ params: { text } }) => {
	const avatar = createAvatar(style, {
		seed: text
	});
	const body = await avatar.toArrayBuffer();
	const etag = generateEtag(Buffer.from(body),{
		weak:false,
	});
	return new Response(body, {
		headers: {
			'Content-Type': 'image/svg+xml',
			'ETag': etag,
			'Cache-Control': 'public, max-age=31536000, immutable'
		}
	});
};