import type { ParamMatcher } from '@sveltejs/kit';
import { UserRegex } from '$lib/expressions';

export const match: ParamMatcher = (param) => {
	return UserRegex.test(param);
};
