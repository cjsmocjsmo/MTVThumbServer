use leptos::prelude::*;

#[component]
pub fn TVShowsListPage() -> impl IntoView {
    // let (count, set_count) = signal(0);
    view! {
        <div class="tvshows-section">
            <div class="tvshows-sectionDiv">
                <a class="tvshows-sectionDivItem" href="">"Action"</a>
                <a class="tvshows-sectionDivItem" href="">"Comedy"</a>
                <a class="tvshows-sectionDivItem" href="">"Fantasy"</a>
                <a class="tvshows-sectionDivItem" href="">"MCU"</a>
                <a class="tvshows-sectionDivItem" href="">"Science"</a>
                <a class="tvshows-sectionDivItem" href="">"SciFi"</a>
                <a class="tvshows-sectionDivItem" href="">"Star Trek"</a>
                <a class="tvshows-sectionDivItem" href="">"Star Wars"</a>
                <a class="tvshows-sectionDivItem" href="">"Westerns"</a>
            </div>
        </div>
        // <p
        //     class:red=move || count.get() % 2 == 1
        // >"Fuck yea"</a>
        // <button
        //     on:click=move |_| *set_count.write() += 1
        //     class:red=move || count.get() % 2 == 1
        // >
        //     "Click me: "
        //     {count}
        // </button>
        // <p>
        //     "Double count: "
        //     {move || count.get() * 2}
            
        // </a>

    }
}
