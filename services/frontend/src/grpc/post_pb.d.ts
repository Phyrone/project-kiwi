// package: de.phyrone.kiwi.publications
// file: post.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";

export class Publication extends jspb.Message {
  getId(): number;
  setId(value: number): void;

  getIsdraft(): boolean;
  setIsdraft(value: boolean): void;

  getTitle(): string;
  setTitle(value: string): void;

  hasContent(): boolean;
  clearContent(): void;
  getContent(): string;
  setContent(value: string): void;

  getUserid(): number;
  setUserid(value: number): void;

  hasCreatedat(): boolean;
  clearCreatedat(): void;
  getCreatedat(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedat(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasUpdatedat(): boolean;
  clearUpdatedat(): void;
  getUpdatedat(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedat(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeletedat(): boolean;
  clearDeletedat(): void;
  getDeletedat(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setDeletedat(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Publication.AsObject;
  static toObject(includeInstance: boolean, msg: Publication): Publication.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Publication, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Publication;
  static deserializeBinaryFromReader(message: Publication, reader: jspb.BinaryReader): Publication;
}

export namespace Publication {
  export type AsObject = {
    id: number,
    isdraft: boolean,
    title: string,
    content: string,
    userid: number,
    createdat?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedat?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deletedat?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

