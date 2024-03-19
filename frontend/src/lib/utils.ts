export async function delay(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export function todo(description?: string): never {
	throw new Error('TODO' + (description ? ': ' + description : ''));
}
