#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

// #[derive(Deserialize, Debug, Clone)]
// struct VanDam {
//     Name: String,
//     HttpThumbPath: String,
// }

// #[component]
// pub fn VanDamPage() -> impl IntoView {
//     let (vandam, set_vandam) = signal(Vec::new());

//     spawn_local(async move {
//         match fetch_vandam().await {
//             Ok(data) => {
//                 log::info!("Fetched VanDam data: {:?}", data); // Debugging log
//                 set_vandam.set(data);
//             },
//             Err(err) => log::error!("Error fetching VanDam data: {:?}", err),
//         }
//     });

//     view! {
//         <div class="vandam-grid">
//             {move || vandam.get().iter().map(|vanda| view! {
//                 <div class="vandam-item">
//                     <img src={vanda.HttpThumbPath.clone()} alt={vanda.Name.clone()} />
//                     <p>{vanda.Name.clone()}</p>
//                 </div>
//             }).collect_view()}
//         </div>
//     }
// }

// async fn fetch_vandam() -> Result<Vec<VanDam>, Error> {
//     let response = reqwest::get("http://10.0.4.41:7777/vandam").await?;
//     let vandam: Vec<VanDam> = response.json().await?;
//     Ok(vandam)
// }

#[derive(Deserialize, Debug, Clone)]
struct Infos {
    Name: String,
    HttpThumbPath: String,
}

#[component]
pub fn VanDamPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_vandam().await {
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
async fn fetch_vandam() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/vandam").await?;
    let vandam: Vec<Infos> = response.json().await?;
    Ok(vandam)
}