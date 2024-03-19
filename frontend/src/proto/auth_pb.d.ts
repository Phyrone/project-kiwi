// package: de.phyrone.kiwi.auth
// file: auth.proto

import * as jspb from 'google-protobuf';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as google_protobuf_api_pb from 'google-protobuf/google/protobuf/api_pb';

export class ValidateSessionRequest extends jspb.Message {
	getToken(): string;
	setToken(value: string): void;

	serializeBinary(): Uint8Array;
	toObject(includeInstance?: boolean): ValidateSessionRequest.AsObject;
	static toObject(
		includeInstance: boolean,
		msg: ValidateSessionRequest
	): ValidateSessionRequest.AsObject;
	static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
	static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
	static serializeBinaryToWriter(message: ValidateSessionRequest, writer: jspb.BinaryWriter): void;
	static deserializeBinary(bytes: Uint8Array): ValidateSessionRequest;
	static deserializeBinaryFromReader(
		message: ValidateSessionRequest,
		reader: jspb.BinaryReader
	): ValidateSessionRequest;
}

export namespace ValidateSessionRequest {
	export type AsObject = {
		token: string;
	};
}

export class ValidateSessionResponse extends jspb.Message {
	hasSession(): boolean;
	clearSession(): void;
	getSession(): ValidateSessionResponse.Session | undefined;
	setSession(value?: ValidateSessionResponse.Session): void;

	hasInvalidsession(): boolean;
	clearInvalidsession(): void;
	getInvalidsession(): ValidateSessionResponse.InvalidSession | undefined;
	setInvalidsession(value?: ValidateSessionResponse.InvalidSession): void;

	getResponseCase(): ValidateSessionResponse.ResponseCase;
	serializeBinary(): Uint8Array;
	toObject(includeInstance?: boolean): ValidateSessionResponse.AsObject;
	static toObject(
		includeInstance: boolean,
		msg: ValidateSessionResponse
	): ValidateSessionResponse.AsObject;
	static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
	static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
	static serializeBinaryToWriter(message: ValidateSessionResponse, writer: jspb.BinaryWriter): void;
	static deserializeBinary(bytes: Uint8Array): ValidateSessionResponse;
	static deserializeBinaryFromReader(
		message: ValidateSessionResponse,
		reader: jspb.BinaryReader
	): ValidateSessionResponse;
}

export namespace ValidateSessionResponse {
	export type AsObject = {
		session?: ValidateSessionResponse.Session.AsObject;
		invalidsession?: ValidateSessionResponse.InvalidSession.AsObject;
	};

	export class Session extends jspb.Message {
		serializeBinary(): Uint8Array;
		toObject(includeInstance?: boolean): Session.AsObject;
		static toObject(includeInstance: boolean, msg: Session): Session.AsObject;
		static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
		static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
		static serializeBinaryToWriter(message: Session, writer: jspb.BinaryWriter): void;
		static deserializeBinary(bytes: Uint8Array): Session;
		static deserializeBinaryFromReader(message: Session, reader: jspb.BinaryReader): Session;
	}

	export namespace Session {
		export type AsObject = {};
	}

	export class InvalidSession extends jspb.Message {
		serializeBinary(): Uint8Array;
		toObject(includeInstance?: boolean): InvalidSession.AsObject;
		static toObject(includeInstance: boolean, msg: InvalidSession): InvalidSession.AsObject;
		static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
		static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
		static serializeBinaryToWriter(message: InvalidSession, writer: jspb.BinaryWriter): void;
		static deserializeBinary(bytes: Uint8Array): InvalidSession;
		static deserializeBinaryFromReader(
			message: InvalidSession,
			reader: jspb.BinaryReader
		): InvalidSession;
	}

	export namespace InvalidSession {
		export type AsObject = {};
	}

	export enum ResponseCase {
		RESPONSE_NOT_SET = 0,
		SESSION = 1,
		INVALIDSESSION = 2
	}
}
