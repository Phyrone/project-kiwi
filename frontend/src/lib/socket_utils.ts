
export async function createWebsocket(address: string, protocols?: string[]): Promise<WrappedWebSocket> {
	return await new Promise((resolve, reject) => {
		const websocket = new WebSocket(address, protocols);
		const wrapped_websocket = new WrappedWebSocket(websocket);

		websocket.onopen = () => resolve(wrapped_websocket);
		websocket.onerror = reject;
	});
}

export class WrappedWebSocket {

	readonly message_buffer: any[] = [];
	readonly request_buffer: { resolve: (any: MessageEvent<any>) => void, reject: (error: Error) => void }[] = [];
	closed: CloseEvent | undefined = undefined;

	private _on_close(event: CloseEvent) {
		this.closed = event;
		console.log('this:', this);
		for (const request of this.request_buffer) {
			request.reject(new Error('Connection closed'));
		}
	}

	private _incomming_message(message: MessageEvent<any>) {
		const request = this.request_buffer.shift();
		if (request) {
			request.resolve(message);
		} else {
			this.message_buffer.push(message);
		}
	}

	constructor(readonly socket: WebSocket) {
		socket.onmessage = (event) => this._incomming_message(event);
		socket.onclose = (event) => this._on_close(event);
	}

	public send(data: any) {
		if (typeof data === 'object') {
			data = JSON.stringify(data);
		}
		this.socket.send(data);
	}

	public async receive(): Promise<MessageEvent<any>> {
		const buffered = this.message_buffer.shift();
		if (buffered)
			return buffered;
		else {
			if (this.closed)
				throw new Error('Connection closed');

			return new Promise((resolve, reject) => {
				this.request_buffer.push({ resolve, reject });
			});
		}
	}

}