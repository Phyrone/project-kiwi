import type { RowList, Sql, TransactionSql } from 'postgres';
import jwt, { type GetPublicKeyOrSecret, type Jwt, type Secret } from 'jsonwebtoken';
import { sql } from '$lib/server/database';
import type { SessionData } from '$lib/server/auth';
import secureRandom from 'secure-random';
import { type DbID, is_alphanumeric } from '$lib/utils';
import type { Cookies } from '@sveltejs/kit';
import { redis } from '$lib/server/redis';

export const SESSION_COOKIE = 'session';
export const SESSION_SECRET_LENGTH = 512;

export const SESSION_CACHE_TTL = 300;

export function token_cache_key(token: string): string | undefined {
	if (token.length > 1024 * 128) {
		return undefined;
	}

	const header_index = token.indexOf('.');
	if (header_index === -1) {
		return undefined;
	}
	const kid = JSON.parse(atob(token.slice(0, header_index)))?.kid;
	if (!kid) {
		return undefined;
	} else if (!is_alphanumeric(kid) || kid.length > 128) {
		return undefined;
	}

	return token_cache_key_2(kid, token);
}

export function token_cache_key_2(kid: string, token: string): string {
	return `session:${kid}:${token}`;
}

export async function validate_session_token_cached(
	token: string,
	transaction?: TransactionSql
): Promise<SessionData | undefined> {
	const key = token_cache_key(token);
	if (!key) {
		return undefined;
	}
	const cached = await redis.get(key);
	if (cached) {
		redis.expire(key, SESSION_CACHE_TTL).then();
		return JSON.parse(cached);
	} else {
		const result = await validate_session_token(token, transaction);
		if (result) {
			redis.setEx(key, SESSION_CACHE_TTL, JSON.stringify(result)).then();
			return result;
		} else {
			return undefined;
		}
	}
}

export async function validate_session_token(
	token: string,
	transaction?: TransactionSql
): Promise<SessionData | undefined> {
	const getter: GetPublicKeyOrSecret = async (header, callback) => {
		(async () => {
			if (header.kid) {
				const secret = await get_session_secret_from_id(header.kid, transaction);
				if (secret) {
					return secret;
				} else {
					//when undefined is returned, jwt.verify() would accept unsingned tokens which is an serve security issue
					throw new Error('could not find secret by kid');
				}
			} else {
				//when undefined is returned, jwt.verify() would accept unsingned tokens which is an serve security issue
				throw new Error('no kid specified in header');
			}
		})()
			.then((result) => {
				callback(null, result);
			})
			.catch((error) => {
				callback(error, undefined);
			});
	};

	const result = await new Promise<Jwt>((resolve, reject) => {
		jwt.verify(token, getter, { complete: true }, (error, decoded) => {
			if (error) {
				reject(error);
			} else if (decoded) {
				resolve(decoded);
			} else {
				reject(new Error('got neither error nor a value from jwt.verify()'));
			}
		});
	}).catch((e) => {
		console.error('error while validating token', e);
		return undefined;
	});

	if (!result) {
		return undefined;
	}
	const { header, payload } = result;

	if (header.kid === payload['user_id'].toString()) {
		return payload as SessionData;
	} else {
		return undefined;
	}
}

export async function get_session_secret_from_id(
	id: string,
	transaction?: TransactionSql
): Promise<Secret | undefined> {
	const database: Sql = transaction || sql;
	const db_result: RowList<{ session_secret: Buffer }[]> = await database`SELECT session_secret
                                                                          FROM "user"
                                                                          WHERE id = ${id}`;
	if (db_result.length === 0) {
		return undefined;
	}
	return db_result[0].session_secret;
}

export function create_session_token_from_secret(
	session_data: SessionData,
	secret: Secret
): string {
	const token = jwt.sign(session_data, secret, {
		algorithm: 'HS512',
		keyid: session_data.user_id.toString(),
		noTimestamp: false,
		mutatePayload: false
	});
	redis
		.setEx(
			token_cache_key_2(session_data.user_id as string, token),
			SESSION_CACHE_TTL,
			JSON.stringify(session_data)
		)
		.then();
	return token;
}

export async function create_new_session_secret(
	user_id: DbID,
	transactionSql?: TransactionSql
): Promise<Secret> {
	const new_secret = secureRandom(SESSION_SECRET_LENGTH, { type: 'Buffer' });
	const database: Sql = transactionSql || sql;
	const result: RowList<
		{
			session_secret: Buffer;
		}[]
	> = await database`UPDATE "user"
                        SET session_secret = ${new_secret}
                        WHERE id = ${user_id}
                        RETURNING session_secret`;
	if (result.length === 0) {
		throw new Error('regarding user not found');
	}
	return result[0].session_secret;
}

export function set_session_cookie(cookies: Cookies, token: string, timeout?: Date) {
	cookies.set(SESSION_COOKIE, token, {
		httpOnly: true,
		sameSite: 'strict',
		secure: true,
		expires: timeout,
		path: '/'
	});
}

export async function get_session_from_request(
	cookies: Cookies,
	request: Request
): Promise<SessionData | undefined> {
	const auth_header = request.headers.get('Authorization');
	let token: string | undefined = undefined;
	if (auth_header && auth_header.startsWith('Bearer ')) {
		token = auth_header.slice(7);
	} else {
		token = cookies.get(SESSION_COOKIE);
	}
	if (token) {
		return await validate_session_token_cached(token);
	} else {
		return undefined;
	}
}

export async function logout_all(user_id: DbID) {
	create_new_session_secret(user_id).then();
	for await (const member of redis.scanIterator({
		MATCH: `session:${user_id}:*`
	})) {
		redis.del(member).then();
	}
}
