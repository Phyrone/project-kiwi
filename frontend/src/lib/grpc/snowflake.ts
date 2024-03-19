import { credentials } from '@grpc/grpc-js';
import { SnowflakeServiceClient } from '$proto/snowflake_grpc_pb';
import { SnowflakeRequest, SnowflakeResponse } from '$proto/snowflake_pb';

const snowflakeServiceClient = new SnowflakeServiceClient('', credentials.createInsecure());

let buffer: number[] = [];

export async function get_snowflake(): Promise<number> {
	if (buffer.length === 0) {
		const request = new SnowflakeRequest();
		request.setCount(1024);
		const response: SnowflakeResponse = await snowflakeServiceClient.getSnowflakes(request);
		buffer = [...buffer, ...response.getSnowflakesList()];
		return buffer.pop() as number;
	} else {
		return buffer.pop() as number;
	}
}
