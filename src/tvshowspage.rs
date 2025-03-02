use leptos::prelude::*;

#[component]
pub fn TVShowsListPage() -> impl IntoView {
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
    }
}
