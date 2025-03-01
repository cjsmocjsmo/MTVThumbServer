use leptos::prelude::*;

#[component]
pub fn TVShowsListPage() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <p
            class:red=move || count.get() % 2 == 1
        >"Fuck yea"</p>
        <button
            on:click=move |_| *set_count.write() += 1
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
            
        </p>
    }
}
