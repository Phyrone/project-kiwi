import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { LoginRequestUserPassSchema } from '$lib/schemas';
import type { LoginRequestUserPass } from '$lib/request_types';
import { respond_known_errors } from '$lib/RFC9457';
import { tryLoginUserPass } from '$lib/server/auth';

export const POST: RequestHandler = async ({ request }) => {
	try {
		const request_data_unvalidated = await request.json();
		const request_data: LoginRequestUserPass = await LoginRequestUserPassSchema.validateAsync(
			request_data_unvalidated,
			{
				abortEarly: false,
				allowUnknown: true,
				cache: true
			}
		);

		const data = tryLoginUserPass(request_data.user, request_data.password);

		return json(
			{
				login: data
			},
			{
				status: 202
			}
		);
	} catch (error) {
		return respond_known_errors(error);
	}
};
