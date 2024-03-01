use leptos::*;
use leptos_router::*;
use crate::components::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
        
                <div>NavBar</div>
        </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
        </Routes>
            </main>
        </Router>
    }
}