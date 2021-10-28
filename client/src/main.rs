// https://github.com/elliptic-email/rust-grpc-web/blob/master/examples/yew-wasm-client/src/main.rs

mod components;
mod state;
pub mod quotes {
	include!(concat!(env!("OUT_DIR"), concat!("/chat.rs")));
}
use components::app::App;

use log::Level;
use sycamore::template;
fn main() {
	console_log::init_with_level(Level::Debug).expect("error initializing logger");
	sycamore::render(|| {
		template! {
				App()
		}
	});
}
