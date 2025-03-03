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
pub fn TinkerBellPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_tinkerbell().await {
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
async fn fetch_tinkerbell() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/tinkerbell").await?;
    let tinkerbell: Vec<Infos> = response.json().await?;
    Ok(tinkerbell)
}

#[component]
pub fn TomCruisePage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_tomcruise().await {
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
async fn fetch_tomcruise() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/tomcruise").await?;
    let tomcruise: Vec<Infos> = response.json().await?;
    Ok(tomcruise)
}

#[component]
pub fn TransformersPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_transformers().await {
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
async fn fetch_transformers() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/transformers").await?;
    let transformers: Vec<Infos> = response.json().await?;
    Ok(transformers)
}

#[component]
pub fn TremorsPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_tremors().await {
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
async fn fetch_tremors() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/tremors").await?;
    let tremors: Vec<Infos> = response.json().await?;
    Ok(tremors)
}

#[component]
pub fn TheRockPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_therock().await {
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
async fn fetch_therock() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/therock").await?;
    let therock: Vec<Infos> = response.json().await?;
    Ok(therock)
}