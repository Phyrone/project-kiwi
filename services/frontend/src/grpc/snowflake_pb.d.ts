// package: de.phyrone.kiwi.snowflake
// file: snowflake.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

export class SnowflakesRequest extends jspb.Message {
  getCount(): number;
  setCount(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SnowflakesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SnowflakesRequest): SnowflakesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SnowflakesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SnowflakesRequest;
  static deserializeBinaryFromReader(message: SnowflakesRequest, reader: jspb.BinaryReader): SnowflakesRequest;
}

export namespace SnowflakesRequest {
  export type AsObject = {
    count: number,
  }
}

export class SnowflakesResponse extends jspb.Message {
  clearSnowflakesList(): void;
  getSnowflakesList(): Array<number>;
  setSnowflakesList(value: Array<number>): void;
  addSnowflakes(value: number, index?: number): number;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SnowflakesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SnowflakesResponse): SnowflakesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SnowflakesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SnowflakesResponse;
  static deserializeBinaryFromReader(message: SnowflakesResponse, reader: jspb.BinaryReader): SnowflakesResponse;
}

export namespace SnowflakesResponse {
  export type AsObject = {
    snowflakesList: Array<number>,
  }
}

export class Snowflake extends jspb.Message {
  getSnowflake(): number;
  setSnowflake(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Snowflake.AsObject;
  static toObject(includeInstance: boolean, msg: Snowflake): Snowflake.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Snowflake, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Snowflake;
  static deserializeBinaryFromReader(message: Snowflake, reader: jspb.BinaryReader): Snowflake;
}

export namespace Snowflake {
  export type AsObject = {
    snowflake: number,
  }
}

