use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos_router::{components::*, path};

mod moviespage;
use crate::moviespage::MovCatListPage;

mod tvshowspage;
use crate::tvshowspage::TVShowsListPage;

mod searchpage;
use crate::searchpage::SearchPage;

fn main() {
	console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar />
            <main>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("/") view=MovCatListPage />
                    <Route path=path!("/tvshows") view=TVShowsListPage />
                    <Route path=path!("/search") view=SearchPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <a href="/" class="navItem">"Movies"</a>
            <a href="/tvshows" class="navItem">"TV Shows"</a>
            <a href="/search" class="navItem">"Search"</a>
        </nav>
    }
}