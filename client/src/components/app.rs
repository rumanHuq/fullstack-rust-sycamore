use crate::state::AppState;
use sycamore::context::{ContextProvider, ContextProviderProps};
use sycamore::prelude::*;
use sycamore::reactive::use_context;
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
