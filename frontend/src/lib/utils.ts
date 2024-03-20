export type DbID = number | string;

export async function delay(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export function todo(description?: string): never {
	throw new Error('TODO' + (description ? ': ' + description : ''));
}

export function is_alphanumeric(str) {
	return /^[a-zA-Z0-9]+$/.test(str);
}
