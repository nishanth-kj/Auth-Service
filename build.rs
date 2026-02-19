fn main() {
    tonic_build::configure()
        .out_dir("src/grpc/traits")     
        .compile(
            &["proto/contracts/test.proto"],
            &["proto"],
        )
        .unwrap();
}