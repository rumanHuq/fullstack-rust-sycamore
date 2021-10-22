fn main() {
	rust_grpc_web::configure()
		.compile(&["../proto/chat.proto"], &["../proto/"])
		.unwrap();
}
