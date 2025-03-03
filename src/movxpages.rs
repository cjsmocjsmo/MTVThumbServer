#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct XMen {
    Name: String,
    HttpThumbPath: String,
}

#[component]
pub fn XMenPage() -> impl IntoView {
    let (xmen, set_xmen) = signal(Vec::new());

    spawn_local(async move {
        match fetch_xmen().await {
            Ok(data) => {
                log::info!("Fetched X-Men data: {:?}", data); // Debugging log
                set_xmen.set(data);
            },
            Err(err) => log::error!("Error fetching X-Men data: {:?}", err),
        }
    });

    view! {
        <div class="xmen-grid">
            {move || xmen.get().iter().map(|xman| view! {
                <div class="xmen-item">
                    <img src={xman.HttpThumbPath.clone()} alt={xman.Name.clone()} />
                    <p>{xman.Name.clone()}</p>
                </div>
            }).collect_view()}
        </div>
    }
}

async fn fetch_xmen() -> Result<Vec<XMen>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/xmen").await?;
    let xmen: Vec<XMen> = response.json().await?;
    Ok(xmen)
}