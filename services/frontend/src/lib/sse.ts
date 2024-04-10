import Joi from 'joi';

export type ServerSendEvent = {
	id?: string;
	event?: string;
	data: unknown;
};

const EventSchema = Joi.object<ServerSendEvent>({
	id: Joi.string().optional(),
	event: Joi.string().min(1).not('\n').optional(),
	data: Joi.any().required()
});

export type SSEHandler = (stream: SSEStream) => Promise<void> | void;

export interface SSEStream {
	onClose?: () => void;
	isClosed: boolean;

	send(event: ServerSendEvent): void;
}

class SSEStreamImpl implements SSEStream {
	//@ts-nocheck
	controller: ReadableStreamController<any>;

	onClose?: () => void;
	isClosed = false;

	constructor(readonly handler: SSEHandler) {
		//@ts-ignore
		this.controller = undefined;
	}

	send(event: ServerSendEvent): boolean {
		if (this.isClosed) {
			return false;
		}

		const { error, value: validated_event } = EventSchema.validate(event, {
			cache: true,
			allowUnknown: true,
			stripUnknown: true
		});
		if (error) {
			throw new Error(`Invalid Argument: ${error.message}`);
		}

		let message = '';
		for (const [key, value] of Object.entries(validated_event)) {
			if (value === undefined) continue;
			if (typeof value === 'string' && !value.includes('\n')) {
				message += `${key}: ${value}\n`;
			} else {
				message += `${key}: ${JSON.stringify(value)}\n`;
			}
		}
		message += '\n';

		// @ts-ignore
		this.controller.enqueue(message);
		return true;
	}

	async run() {
		await this.handler(this);
	}

	close() {
		this.isClosed = true;
		if (this.onClose) {
			this.onClose();
		}
	}
}

export function sse(handler: SSEHandler): Response {
	const sse = new SSEStreamImpl(handler);
	const stream = new ReadableStream({
		async start(controller) {
			try {
				sse.controller = controller;
				sse.send({
					event: 'sse',
					data: 'start of stream'
				});
				await sse.run();
			} catch (e) {
				controller.error(e);
				console.error(e);
			} finally {
				if (!sse.isClosed) {
					try {
						sse.send({
							event: 'sse',
							data: 'end of stream'
						});
					} catch (e) {
						/* ignored */
					}
					sse.close();
				}
			}
			controller.close();
		},
		cancel() {
			sse.close();
		}
	});
	return new Response(stream, {
		headers: {
			'Content-Type': 'text/event-stream',
			'Cache-Control': 'no-cache',
			Connection: 'keep-alive'
		},
		status: 200
	});
}
