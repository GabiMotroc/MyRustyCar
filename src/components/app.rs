use crate::components::home::Home;
use crate::components::login::Login;
use crate::components::navbar::Navbar;
use leptos::*;
use leptos_router::*;
use stylers::{style, style_str};

#[component]
pub fn App() -> impl IntoView {
    let (class_name, style) = style_str! {"App",
        main{
            justify-content: center;
            align-items: center;
            display: flex;
        }
    };

    view! { class=class_name,
        <style>{style}</style>
        <Router>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/test" view=|| view! { Test }/>
                    <Route path="/login" view=Login/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}
