import type { ServerSendEvent } from '$lib/sse';

export class LiveView {
	constructor(
		readonly intents: string[],
		readonly call: (event: ServerSendEvent) => void
	) {}

	async start() {}

	stop() {}
}
