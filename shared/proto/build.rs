fn main() {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true)
        .compile_well_known_types(true)
        .use_arc_self(false)
        .include_file("mod.rs")
        .out_dir("src/proto")
        .compile(&["../proto/snowflake.proto"], &["../proto"])
        .expect("Failed to compile gRPC definitions!");
}
