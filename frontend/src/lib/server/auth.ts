import { type JwtPayload } from 'jsonwebtoken';
import argon2, { argon2id } from 'argon2';
import Joi from 'joi';
import postgres, { type RowList, type TransactionSql } from 'postgres';
import { sql } from '$lib/server/database';
import {
	create_new_session_secret,
	create_session_token_from_secret,
	SESSION_SECRET_LENGTH
} from '$lib/server/session';
import secureRandom from 'secure-random';
import { type DbID } from '$lib/utils';

export type LoginData = {
	user_id: DbID;
	token: string;
};

export type UserPassLoginError =
	| 'user_is_not_a_email'
	| 'user_not_found'
	| 'password_incorrect'
	| 'user_has_no_password';

export type UserPassLoginResult =
	| {
			ok: true;
			data: LoginData;
	  }
	| {
			ok: false;
			error: UserPassLoginError;
	  };

export type SessionData = {
	user_id: DbID;
} & JwtPayload;

export async function login_user_pass(
	user: string,
	password: string,
	transaction?: TransactionSql
): Promise<UserPassLoginResult> {
	const database = transaction || sql;
	const { error, value: email } = Joi.string().email().validate(user) as {
		error: Error | undefined;
		value: string;
	};
	if (error) {
		return {
			ok: false,
			error: 'user_is_not_a_email'
		};
	}
	const user_select: RowList<
		{
			id: DbID;
			password?: string;
			session_secret?: Buffer;
		}[]
	> = await database`SELECT id, password, session_secret
                     FROM "user"
                     WHERE email = ${email.toLowerCase()}`;
	if (user_select.length === 0) {
		return {
			ok: false,
			error: 'user_not_found'
		};
	}
	const user_data = user_select[0];
	if (!user_data.password) {
		return {
			ok: false,
			error: 'user_has_no_password'
		};
	}
	const verify_result = await argon2.verify(user_data.password, password);
	return verify_result
		? {
				ok: true,
				data: {
					user_id: user_data.id,
					token: create_session_token_from_secret(
						{ user_id: user_data.id, iat: Math.floor(Date.now() / 1000) },
						user_data.session_secret ?? (await create_new_session_secret(user_data.id))
					)
				}
			}
		: {
				ok: false,
				error: 'password_incorrect'
			};
}

export type RegisterResult =
	| {
			ok: true;
			user_id: DbID;
			token: string;
	  }
	| {
			ok: false;
			error: 'user_already_exists' | 'email_invalid';
	  };

export async function register_user(
	user: string,
	password?: string,
	transaction?: TransactionSql
): Promise<RegisterResult> {
	const { error, value: email } = Joi.string().email().validate(user) as {
		error: Error | undefined;
		value: string;
	};
	if (error) {
		return {
			ok: false,
			error: 'email_invalid'
		};
	}
	if (password) {
		password = await argon2.hash(password, {
			type: argon2id
		});
	}
	const new_session_secret = secureRandom(SESSION_SECRET_LENGTH, { type: 'Buffer' });
	const database = transaction || sql;

	try {
		const result: RowList<
			{
				id: DbID;
			}[]
		> = await database`INSERT INTO "user"(created_at, email, password, session_secret)
                          values (NOW(), ${email.toLowerCase()}, ${password || null}, ${new_session_secret})
                          RETURNING id`;

		const user_id = result[0].id;

		return {
			ok: true,
			user_id,
			token: create_session_token_from_secret(
				{
					user_id,
					iat: Math.floor(Date.now() / 1000)
				},
				Buffer.from(new_session_secret)
			)
		};
	} catch (e) {
		console.error('pg error', postgres.PostgresError);
		if (
			typeof e === 'object' &&
			e instanceof postgres.PostgresError &&
			e.code === '23505' &&
			e.table_name === 'user' &&
			e.constraint_name === 'user_email_key'
		) {
			return {
				ok: false,
				error: 'user_already_exists'
			};
		} else {
			throw e;
		}
	}
}
