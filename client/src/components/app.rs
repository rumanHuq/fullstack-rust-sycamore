// https://github.com/elliptic-email/rust-grpc-web/blob/master/examples/seed-wasm-client/src/lib.rs
use crate::{
	quotes::{chat_client, User},
	state::AppState,
};
use log::debug;
use sycamore::context::{ContextProvider, ContextProviderProps};
use sycamore::prelude::*;
use sycamore::reactive::use_context;
use wasm_bindgen_futures::spawn_local;
#[component(Counter<G>)]
fn counter(class: &'static str) -> Template<G> {
	let state = use_context::<Signal<AppState>>();

	template! {
			p(class=class) {
					"Value: "
					(state.get().counter)
			}
	}
}

#[component(Controls<G>)]
pub fn controls() -> Template<G> {
	let state = use_context::<Signal<AppState>>();
	let grpc_client = chat_client::Chat::new("http://127.0.0.1:9999".into());

	cloned!(() => {
		spawn_local(async move {
			let resp = grpc_client.say_hello(User {name: String::from("User"), id: 1}).await;
			debug!("{:?}", resp);
		})
	});

	let increment = cloned!((state) => move |_| state.set(state.get().incr()));
	let decrement = cloned!((state) => move |_| state.set(state.get().decr()));
	let reset = cloned!((state) => move |_| state.set(state.get().reset()));

	template! {
		button(class="increment", on:click=increment) {
						"Increment"
				}
		button(class="decrement", on:click=decrement) {
				"Decrement"
		}
		button(class="reset", on:click=reset) {
				"Reset"
		}
	}
}

#[component(App<G>)]
pub fn app() -> Template<G> {
	let counter = Signal::new(AppState::default());

	template! {
		ContextProvider(ContextProviderProps {
			value: counter,
			children: move || {
				template! {
					div {
							h2(class="header") {
								"Counter demo"
							}
							Counter("poop")
							Controls()
					}
				}
			}
		})
	}
}
