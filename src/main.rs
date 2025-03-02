use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos_router::{components::*, path};

mod moviespage;
use crate::moviespage::MovCatListPage;

mod tvshowspage;
use crate::tvshowspage::TVShowsListPage;

mod searchpage;
use crate::searchpage::SearchPage;

mod movactionpage;
use crate::movactionpage::ActionPage;

mod movarnoldpage;
use crate::movarnoldpage::ArnoldPage;

mod movbruceleepage;
use crate::movbruceleepage::BruceLeePage;

mod movbrucewillispage;
use crate::movbrucewillispage::BruceWillisPage;

mod movbuzzpage;
use crate::movbuzzpage::BuzzPage;

fn main() {
	console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar />
            <Header />
            <main>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("/") view=MovCatListPage />
                    <Route path=path!("/tvshows") view=TVShowsListPage />
                    <Route path=path!("/search") view=SearchPage />
                    <Route path=path!("/action") view=ActionPage />
                    <Route path=path!("/arnold") view=ArnoldPage />
                    <Route path=path!("/brucelee") view=BruceLeePage />
                    <Route path=path!("/brucewillis") view=BruceWillisPage />
                    <Route path=path!("/buzz") view=BuzzPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <h1 class="header">"MTV"</h1>
        </header>
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