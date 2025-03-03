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
pub fn SciFiPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_scifi().await {
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
async fn fetch_scifi() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/scifi").await?;
    let scifi: Vec<Infos> = response.json().await?;
    Ok(scifi)
}

#[component]
pub fn StalonePage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_stalone().await {
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
async fn fetch_stalone() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/stalone").await?;
    let stalone: Vec<Infos> = response.json().await?;
    Ok(stalone)
}

// #[component]
// pub fn StarTrekPage() -> impl IntoView {
//     view! {
//         <h1>"StarTrek Page"</h1>
//     }
// }
#[component]
pub fn StarTrekPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_startrek().await {
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
async fn fetch_startrek() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/startrek").await?;
    let startrek: Vec<Infos> = response.json().await?;
    Ok(startrek)
}

// #[component]
// pub fn StarWarsPage() -> impl IntoView {
//     view! {
//         <h1>"StarWars Page"</h1>
//     }
// }
#[component]
pub fn StarWarsPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_starwars().await {
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
async fn fetch_starwars() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/starwars").await?;
    let starwars: Vec<Infos> = response.json().await?;
    Ok(starwars)
}

// #[component]
// pub fn SuperHeroesPage() -> impl IntoView {
//     view! {
//         <h1>"SuperHeroes Page"</h1>
//     }
// }
#[component]
pub fn SuperHerosPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_superheros().await {
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
async fn fetch_superheros() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/superheros").await?;
    let superheros: Vec<Infos> = response.json().await?;
    Ok(superheros)
}