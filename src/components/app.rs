use leptos::*;
use leptos::svg::path;
use leptos_router::*;
use crate::components::home::Home;
use crate::components::navbar::Navbar;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/test" view=|| view! { Test }/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}