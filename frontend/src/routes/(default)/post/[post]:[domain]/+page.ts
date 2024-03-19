import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params: { post, domain }, fetch }) => {
	let post_data = await fetch(`/api/v1/posts/${domain}@${post}`).then((r) => r.json());
};
