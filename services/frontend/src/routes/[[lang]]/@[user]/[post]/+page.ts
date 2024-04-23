import type { PageLoad } from './$types';
import i64b from 'int64-buffer';

export const load: PageLoad = async ({ params: { post } }) => {

	const post_id = BigInt(new i64b.Uint64BE(post, 36).toString());

	return {
		post_id,

	};
};
