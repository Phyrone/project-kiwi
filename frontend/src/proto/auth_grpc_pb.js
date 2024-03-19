// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var auth_pb = require('./auth_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_api_pb = require('google-protobuf/google/protobuf/api_pb.js');

function serialize_de_phyrone_kiwi_auth_ValidateSessionRequest(arg) {
	if (!(arg instanceof auth_pb.ValidateSessionRequest)) {
		throw new Error('Expected argument of type de.phyrone.kiwi.auth.ValidateSessionRequest');
	}
	return Buffer.from(arg.serializeBinary());
}

function deserialize_de_phyrone_kiwi_auth_ValidateSessionRequest(buffer_arg) {
	return auth_pb.ValidateSessionRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_de_phyrone_kiwi_auth_ValidateSessionResponse(arg) {
	if (!(arg instanceof auth_pb.ValidateSessionResponse)) {
		throw new Error('Expected argument of type de.phyrone.kiwi.auth.ValidateSessionResponse');
	}
	return Buffer.from(arg.serializeBinary());
}

function deserialize_de_phyrone_kiwi_auth_ValidateSessionResponse(buffer_arg) {
	return auth_pb.ValidateSessionResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

var AuthServiceService = (exports.AuthServiceService = {
	validateSession: {
		path: '/de.phyrone.kiwi.auth.AuthService/ValidateSession',
		requestStream: false,
		responseStream: false,
		requestType: auth_pb.ValidateSessionRequest,
		responseType: auth_pb.ValidateSessionResponse,
		requestSerialize: serialize_de_phyrone_kiwi_auth_ValidateSessionRequest,
		requestDeserialize: deserialize_de_phyrone_kiwi_auth_ValidateSessionRequest,
		responseSerialize: serialize_de_phyrone_kiwi_auth_ValidateSessionResponse,
		responseDeserialize: deserialize_de_phyrone_kiwi_auth_ValidateSessionResponse
	}
});

exports.AuthServiceClient = grpc.makeGenericClientConstructor(AuthServiceService);
