import type { RowList } from 'postgres';
import { sql } from '$lib/server/database';
import argon2 from 'argon2';
import {  } from '$lib/server/session';

export type LoginData = {
	user_id: number;
	token: string;
};

export async function tryLoginUserPass(
	user: string,
	password: string
): Promise<LoginData | undefined> {

	return undefined;
}
