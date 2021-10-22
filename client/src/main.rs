// https://github.com/elliptic-email/rust-grpc-web/blob/master/examples/yew-wasm-client/src/main.rs

mod components;
mod state;
pub mod quotes {
	include!(concat!(env!("OUT_DIR"), concat!("/chat.rs")));
}
use components::app::App;
use quotes::{chat_client, Message, User};

use sycamore::template;
fn main() {
	sycamore::render(|| {
		template! {
				App()
		}
	});
}
