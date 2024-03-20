import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { respond_known_errors } from '$lib/RFC9457';
import { RegisterRequestSchema } from '$lib/schemas';
import { register_user } from '$lib/server/auth';

export const POST: RequestHandler = async ({ request }) => {
	try {
		const unvalidated_request_data = await request.json();
		const validated_request_data =
			await RegisterRequestSchema.validateAsync(unvalidated_request_data);
		const { user, password } = validated_request_data;

		const result = await register_user(user, password);
		if (result.ok) {
			return json(result);
		} else {
			return json({ error: result.error }, { status: 400 });
		}
	} catch (e) {
		return respond_known_errors(e);
	}
};
