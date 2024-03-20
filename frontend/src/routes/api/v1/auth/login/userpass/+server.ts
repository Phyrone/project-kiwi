import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { LoginRequestUserPassSchema } from '$lib/schemas';
import type { LoginRequestUserPass } from '$lib/request_types';
import { respond_known_errors } from '$lib/RFC9457';
import { login_user_pass } from '$lib/server/auth';
import { set_session_cookie } from '$lib/server/session';

export const POST: RequestHandler = async ({ request, cookies }) => {
	try {
		const request_data_unvalidated = await request.json();
		const request_data: LoginRequestUserPass = await LoginRequestUserPassSchema.validateAsync(
			request_data_unvalidated,
			{
				abortEarly: false,
				allowUnknown: true,
				stripUnknown: true,
				cache: true
			}
		);

		const result = await login_user_pass(request_data.user, request_data.password);
		if (result.ok) {
			const data = result.data;
			set_session_cookie(cookies, data.token);
			return json(
				{
					login: data
				},
				{
					status: 202
				}
			);
		} else {
			return json(
				{
					error: result.error
				},
				{
					status: 401
				}
			);
		}
	} catch (error) {
		return respond_known_errors(error);
	}
};
