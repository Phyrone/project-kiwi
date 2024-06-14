OUTPUT_DIR=src/grpc
INPUT_DIR=../../proto

if [ -d "$OUTPUT_DIR" ]; then
  rm -r $OUTPUT_DIR
fi
mkdir -p src/grpc

#protoc -I ../../proto snowflake.proto post.proto  --grpc-web_out=import_style=commonjs,mode=grpcweb:src/grpc

grpc_tools_node_protoc \
  --js_out=import_style=commonjs,binary:$OUTPUT_DIR \
  --grpc_out=grpc_js:$OUTPUT_DIR \
  --plugin=protoc-gen-grpc=`which grpc_tools_node_protoc_plugin` \
  -I $INPUT_DIR \
  $INPUT_DIR/*.proto
protoc \
  --plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts \
  --ts_out=$OUTPUT_DIR \
  -I $INPUT_DIR \
  $INPUT_DIR/*.proto