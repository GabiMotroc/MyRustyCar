mod components;

use leptos::*;
use crate::components::app::App;

fn main() {
    mount_to_body(|| {
        view! { <App/> }
    })
}
