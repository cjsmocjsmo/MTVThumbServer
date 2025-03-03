#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Infos {
    Name: String,
    HttpThumbPath: String,
}

#[component]
pub fn XMenPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_xmen().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-grid">
            {move || infos.get().iter().map(|info| view! {
                <div class="mov-item">
                    <img src={info.HttpThumbPath.clone()} alt={info.Name.clone()} />
                    <p>{info.Name.clone()}</p>
                </div>
            }).collect_view()}
        </div>
    }
}
async fn fetch_xmen() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/xmen").await?;
    let xmen: Vec<Infos> = response.json().await?;
    Ok(xmen)
}