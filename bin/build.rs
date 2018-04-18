extern crate protoc_grpcio;

fn main() {
    protoc_grpcio::compile_grpc_protos(
        &[
            "proto/edges.proto",
            "proto/metadata.proto",
            "proto/queries.proto",
            "proto/request.proto",
            "proto/response.proto",
            "proto/service.proto",
            "proto/vertices.proto",
        ],
        &["."],
        "src/common/autogen"
    ).expect("failed to compile gRPC definitions");
}
