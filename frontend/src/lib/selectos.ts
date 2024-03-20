import { UserRegex } from '$lib/expressions';

export type UserSelector = {
	username: string;
	//discriminator is a number	and can have an arbitrary amount of leading zeroes for formatting
	discriminator?: number;
	domain?: string;
};

export function parse_user_selector(string: string): UserSelector | undefined {
	const regex = UserRegex.exec(string);
	if (!regex) return undefined;
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	const [_, username, discriminator, domain] = regex;

	return {
		username: username.substring(1),
		discriminator: discriminator ? parseInt(discriminator.substring(1)) : undefined,
		domain: domain ? domain.substring(1) : undefined
	};
}

export function stringify_user_selector(selector: UserSelector): string {
	return `@${selector.username}${selector.discriminator ? `#${selector.discriminator}` : ''}${selector.domain ? `@${selector.domain}` : ''}`;
}
