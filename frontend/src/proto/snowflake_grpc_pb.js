// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var snowflake_pb = require('./snowflake_pb.js');

function serialize_de_phyrone_kiwi_snowflake_SnowflakeRequest(arg) {
	if (!(arg instanceof snowflake_pb.SnowflakeRequest)) {
		throw new Error('Expected argument of type de.phyrone.kiwi.snowflake.SnowflakeRequest');
	}
	return Buffer.from(arg.serializeBinary());
}

function deserialize_de_phyrone_kiwi_snowflake_SnowflakeRequest(buffer_arg) {
	return snowflake_pb.SnowflakeRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_de_phyrone_kiwi_snowflake_SnowflakeResponse(arg) {
	if (!(arg instanceof snowflake_pb.SnowflakeResponse)) {
		throw new Error('Expected argument of type de.phyrone.kiwi.snowflake.SnowflakeResponse');
	}
	return Buffer.from(arg.serializeBinary());
}

function deserialize_de_phyrone_kiwi_snowflake_SnowflakeResponse(buffer_arg) {
	return snowflake_pb.SnowflakeResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

var SnowflakeServiceService = (exports.SnowflakeServiceService = {
	getSnowflakes: {
		path: '/de.phyrone.kiwi.snowflake.SnowflakeService/GetSnowflakes',
		requestStream: false,
		responseStream: false,
		requestType: snowflake_pb.SnowflakeRequest,
		responseType: snowflake_pb.SnowflakeResponse,
		requestSerialize: serialize_de_phyrone_kiwi_snowflake_SnowflakeRequest,
		requestDeserialize: deserialize_de_phyrone_kiwi_snowflake_SnowflakeRequest,
		responseSerialize: serialize_de_phyrone_kiwi_snowflake_SnowflakeResponse,
		responseDeserialize: deserialize_de_phyrone_kiwi_snowflake_SnowflakeResponse
	}
});

exports.SnowflakeServiceClient = grpc.makeGenericClientConstructor(SnowflakeServiceService);
