// package: de.phyrone.kiwi.snowflake
// asset: snowflake.proto

import * as jspb from 'google-protobuf';

export class SnowflakeRequest extends jspb.Message {
	getCount(): number;
	setCount(value: number): void;

	serializeBinary(): Uint8Array;
	toObject(includeInstance?: boolean): SnowflakeRequest.AsObject;
	static toObject(includeInstance: boolean, msg: SnowflakeRequest): SnowflakeRequest.AsObject;
	static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
	static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
	static serializeBinaryToWriter(message: SnowflakeRequest, writer: jspb.BinaryWriter): void;
	static deserializeBinary(bytes: Uint8Array): SnowflakeRequest;
	static deserializeBinaryFromReader(
		message: SnowflakeRequest,
		reader: jspb.BinaryReader
	): SnowflakeRequest;
}

export namespace SnowflakeRequest {
	export type AsObject = {
		count: number;
	};
}

export class SnowflakeResponse extends jspb.Message {
	clearSnowflakesList(): void;
	getSnowflakesList(): Array<number>;
	setSnowflakesList(value: Array<number>): void;
	addSnowflakes(value: number, index?: number): number;

	serializeBinary(): Uint8Array;
	toObject(includeInstance?: boolean): SnowflakeResponse.AsObject;
	static toObject(includeInstance: boolean, msg: SnowflakeResponse): SnowflakeResponse.AsObject;
	static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
	static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
	static serializeBinaryToWriter(message: SnowflakeResponse, writer: jspb.BinaryWriter): void;
	static deserializeBinary(bytes: Uint8Array): SnowflakeResponse;
	static deserializeBinaryFromReader(
		message: SnowflakeResponse,
		reader: jspb.BinaryReader
	): SnowflakeResponse;
}

export namespace SnowflakeResponse {
	export type AsObject = {
		snowflakesList: Array<number>;
	};
}
