import { UserRegex } from '$lib/expressions';

export type UserSelector = {
	username: string;
	//discriminator is a number	and can have an arbitrary amount of leading zeroes for formatting
	discriminator?: number;
	host?: string;
};

export function parse_user_selector(string: string): UserSelector | undefined {
	const regex = UserRegex.exec(string);
	if (!regex) return undefined;
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	const [_full, _username_raw, username, _discriminator_string_raw, discriminator_string, _domain_raw, domain] = regex;

	let discriminator: number | undefined = undefined;
	if (discriminator_string) {
		discriminator = parseInt(discriminator_string);
		if (!isFinite(discriminator)) return undefined;
		//limit to 16 bit (unsigned) integer
		if (discriminator < 0 || discriminator >= 65536) return undefined;
	}

	return {
		username: username,
		discriminator: discriminator,
		host: domain ? domain : undefined
	};
}

export function stringify_user_selector(selector: UserSelector): string {
	return `@${selector.username}${selector.discriminator ? `#${selector.discriminator}` : ''}${selector.host ? `@${selector.host}` : ''}`;
}
