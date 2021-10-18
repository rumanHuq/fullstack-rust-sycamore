mod components;
use components::app::App;
use sycamore::template;

fn main() {
  sycamore::render(|| {
    template! {
      App()
    }
  });
}
