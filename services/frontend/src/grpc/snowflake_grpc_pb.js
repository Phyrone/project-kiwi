// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var snowflake_pb = require('./snowflake_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_de_phyrone_kiwi_snowflake_Snowflake(arg) {
  if (!(arg instanceof snowflake_pb.Snowflake)) {
    throw new Error('Expected argument of type de.phyrone.kiwi.snowflake.Snowflake');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_de_phyrone_kiwi_snowflake_Snowflake(buffer_arg) {
  return snowflake_pb.Snowflake.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_de_phyrone_kiwi_snowflake_SnowflakesRequest(arg) {
  if (!(arg instanceof snowflake_pb.SnowflakesRequest)) {
    throw new Error('Expected argument of type de.phyrone.kiwi.snowflake.SnowflakesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_de_phyrone_kiwi_snowflake_SnowflakesRequest(buffer_arg) {
  return snowflake_pb.SnowflakesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_de_phyrone_kiwi_snowflake_SnowflakesResponse(arg) {
  if (!(arg instanceof snowflake_pb.SnowflakesResponse)) {
    throw new Error('Expected argument of type de.phyrone.kiwi.snowflake.SnowflakesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_de_phyrone_kiwi_snowflake_SnowflakesResponse(buffer_arg) {
  return snowflake_pb.SnowflakesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_google_protobuf_Empty(arg) {
  if (!(arg instanceof google_protobuf_empty_pb.Empty)) {
    throw new Error('Expected argument of type google.protobuf.Empty');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_google_protobuf_Empty(buffer_arg) {
  return google_protobuf_empty_pb.Empty.deserializeBinary(new Uint8Array(buffer_arg));
}


var SnowflakeServiceService = exports.SnowflakeServiceService = {
  getSnowflake: {
    path: '/de.phyrone.kiwi.snowflake.SnowflakeService/GetSnowflake',
    requestStream: false,
    responseStream: false,
    requestType: google_protobuf_empty_pb.Empty,
    responseType: snowflake_pb.Snowflake,
    requestSerialize: serialize_google_protobuf_Empty,
    requestDeserialize: deserialize_google_protobuf_Empty,
    responseSerialize: serialize_de_phyrone_kiwi_snowflake_Snowflake,
    responseDeserialize: deserialize_de_phyrone_kiwi_snowflake_Snowflake,
  },
  getSnowflakes: {
    path: '/de.phyrone.kiwi.snowflake.SnowflakeService/GetSnowflakes',
    requestStream: false,
    responseStream: false,
    requestType: snowflake_pb.SnowflakesRequest,
    responseType: snowflake_pb.SnowflakesResponse,
    requestSerialize: serialize_de_phyrone_kiwi_snowflake_SnowflakesRequest,
    requestDeserialize: deserialize_de_phyrone_kiwi_snowflake_SnowflakesRequest,
    responseSerialize: serialize_de_phyrone_kiwi_snowflake_SnowflakesResponse,
    responseDeserialize: deserialize_de_phyrone_kiwi_snowflake_SnowflakesResponse,
  },
};

exports.SnowflakeServiceClient = grpc.makeGenericClientConstructor(SnowflakeServiceService);
