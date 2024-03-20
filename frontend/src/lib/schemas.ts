import Joi, { type ValidationError } from 'joi';
import { standardizedError } from '$lib/RFC9457';
import { DOCUMENTATION_API_ERRORS_URL } from '$lib/consts';
import type { GetRequestSearchParams, RegisterRequest } from '$lib/request_types';

export function schemaError(error: ValidationError) {
	return standardizedError({
		type: `${DOCUMENTATION_API_ERRORS_URL}common#request-schema`,
		title: 'Invalid Schema',
		detail: error.message,
		status: 400,
		additional: {
			report: error.details
		}
	});
}

export const LoginRequestUserPassSchema = Joi.object({
	user: Joi.string().email({ allowUnicode: true }).required(),
	password: Joi.string().required(),
	remember: Joi.boolean().optional()
});

export const SendMessageRequestSchema = Joi.object({
	message: Joi.string().min(1).max(16_000, 'utf8').required()
});

export const GetRequestSearchParamsSchema = Joi.object<GetRequestSearchParams>({
	since: Joi.string().isoDate().optional(),
	until: Joi.string().isoDate().optional(),
	limit: Joi.number().integer().min(1).optional(),
	offset: Joi.number().integer().min(0).optional(),
	ids: Joi.array().items(Joi.string()).optional(),
	users: Joi.array().items(Joi.string()).optional(),
	watch: Joi.boolean().optional()
});

export const RegisterRequestSchema = Joi.object<RegisterRequest>({
	user: Joi.string().email({ allowUnicode: true }).required(),
	password: Joi.string().min(1).optional()
});

export const LiveSearchParamsSchema = Joi.object({
	intents: Joi.array().items(Joi.string()).optional()
});
