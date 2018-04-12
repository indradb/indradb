extern crate protoc_grpcio;

fn main() {
    protoc_grpcio::compile_grpc_protos(
        &[
            "edges.proto",
            "metadata.proto",
            "queries.proto",
            "request.proto",
            "response.proto",
            "service.proto",
            "vertices.proto",
        ],
        &["proto"],
        "src/common/autogen"
    ).expect("failed to compile gRPC definitions");
}
