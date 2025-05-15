use async_dup::Arc;
use async_lock::{OnceCell, RwLock};
use atoll_common::{Project, Publisher};
use ed25519_dalek::VerifyingKey;
use rocket::{fs::FileServer, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use surrealkv::{Options, Store};
mod db;
pub(crate) use db::*;

mod errors;
pub(crate) use errors::*;

pub(crate) static KV: OnceCell<DbState> = OnceCell::new();

pub(crate) const PUBLISHERS_DB: &str = "PUBLISHERS";
pub(crate) const PROJECTS_DB: &str = "PROJECTS";

#[macro_use]
extern crate rocket;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome<T> {
    Success(T),
    Failure(String),
}

#[get("/projects")]
async fn projects() -> Json<Outcome<Vec<Project>>> {
    if let Ok(projects_raw) = DbState::values(PROJECTS_DB).await {
        let mut projects = Vec::<Project>::default();

        if projects_raw
            .into_iter()
            .try_for_each(|project_bytes| {
                projects.push(bincode::deserialize::<Project>(&project_bytes)?);

                Ok::<(), BackendError>(())
            })
            .is_ok()
        {
            Json(Outcome::Success(projects))
        } else {
            Json(Outcome::Failure("Internal Server Error".to_string()))
        }
    } else {
        Json(Outcome::Failure("Internal Server Error".to_string()))
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if DbState::init().await.is_err() {
        panic!("KV store already initialized!");
    } else {
        let assets_path = concat!(env!("CARGO_WORKSPACE_DIR"), "public");

        let _ = rocket::build()
            .mount("/", FileServer::from(assets_path))
            .mount("/", routes![projects])
            .launch()
            .await?;

        Ok(())
    }
}

/*

pub fn foo() -> Publisher {
    let publisher_address = "5bAvxZRsixc9F6fL1Kzv2jfgCZ1UXrVW5FsWcHfppZUy";
    let publisher_bytes: [u8; 32] = bs58::decode(publisher_address)
        .into_vec()
        .unwrap()
        .try_into()
        .unwrap();
    let publisher_pubkey = VerifyingKey::from_bytes(&publisher_bytes).unwrap();

    let merch = vec![
        Merch {
            supplier: "Global SOL Merch Industries".to_string(),
            image: "/assets/sol-mug.jpg".to_string(),
            name: "Solana Branded Mug".to_string(),
            description: "Lovely Solana Branded Mug for sipping while in the trenches".to_string(),
        },
        Merch {
            supplier: "Global SOL Merch Industries".to_string(),
            image: "/assets/sol-hoodie.jpg".to_string(),
            name: "Solana Branded Hoodie".to_string(),
            description: "Stay warm while in the trenches".to_string(),
        },
        Merch {
            supplier: "Laptop Sticker".to_string(),
            image: "/assets/sol-mug.jpg".to_string(),
            name: "Solana Branded Mug".to_string(),
            description: "Laptop powered by Solana love.".to_string(),
        },
    ];

    Publisher {
        name: "JAMII DAO".to_string(),
        public_key: publisher_pubkey,
        mint: VerifyingKey::from_bytes(&[9u8; 32]).unwrap(),
        merch,
    }
}

pub fn projects(publisher: &Publisher) -> Vec<Project> {
    let phishing1 = Phishing {
        name: "SolanaWalletAdapter".to_string(),
        uri: "https://github.com/Jamii-Dao/SolanaWalletAdapter".to_string(),
        analysis: "This project impersonates the publisher of this library. The creator of the organization added a hyphen between `Jamii` and `Dao` to create a Github organization called `Jamii-Dao` which is similar to the publisher of this library `JamiiDao`. They mirrored the commits of this library to make it seem like the publisher of this library is part of their phishing organization.".to_string(),
        other_uri: vec![
            ("x.com/JamiiDao".to_string(), "x.com/JamiiDao".to_string()),
           ("Phishing Codebase".to_string(), "https://github.com/Jamii-Dao/Solan,aWalletAdapter/".to_string()),
           ("Phishing Website".to_string(), "https://jamiidao.com/".to_string()),
           ("Phishing Meme Coin".to_string(), "https://pump.fun/coin/3ZxaS6sDzdJjiwCRtf3RkWfEw25XeU5bLsvkmkNArxcp".to_string())
        ]
    };

    let project1_name = "wallet-adapter";
    let project1 = Project {
        id: blake3::hash(project1_name.as_bytes()),
        name: project1_name.to_string(),
        logo: "/assets/wallet-adapter-logo.svg".to_string(),
        icon: "/assets/wallet-adapter-icon.svg".to_string(),
        publisher: (
            publisher.short_address(),
            publisher.address(),
            publisher.name.clone(),
        ),
        codebase: "https://github.com/JamiiDao/SolanaWalletAdapter".to_string(),
        website: Some("https://jamiidao.app".to_string()),
        package_uri: "https://crates.io/crates/wallet-adapter".to_string(),
        docs: "https://jamiidao.github.io/SolanaWalletAdapter/".to_string(),
        phishing: vec![phishing1],
        category: ProjectCategory::Wallet,
        language: CodeLanguage::Rust,
        description: "The wallet-adapter library is a Rust crate that performs actions between a Rust WebAssembly frontend and browser wallet extensions that implement the wallet-standard.".to_string()
    };
    let phishing2 = Phishing {
        name: " SolanaPayments".to_string(),
        uri: "https://github.com/Jamii-Dao/SolanaPayments".to_string(),
        analysis: "This project impersonates the publisher of this library. The creator of the organization added a hyphen between `Jamii` and `Dao` to create a Github organization called `Jamii-Dao` which is similar to the publisher of this library `JamiiDao`. They mirrored the commits of this library to make it seem like the publisher of this library is part of their phishing organization.".to_string(),
        other_uri: vec![
            ("x.com/JamiiDao".to_string(), "x.com/JamiiDao".to_string()),
           ("Phishing Codebase".to_string(), "https://github.com/Jamii-Dao/SolanaPayments".to_string()),
           ("Phishing Website".to_string(), "https://jamiidao.com/".to_string()),
           ("Phishing Meme Coin".to_string(), "https://pump.fun/coin/3ZxaS6sDzdJjiwCRtf3RkWfEw25XeU5bLsvkmkNArxcp".to_string())
        ]
    };

    let project2_name = "solana-payments";
    let project2 = Project {
        id: blake3::hash(project2_name.as_bytes()),
        name: project2_name.to_string(),
        logo: "/assets/solana-payments-logo.svg".to_string(),
        icon: "/assets/solana-payments-icon.svg".to_string(),
        publisher: (
            publisher.short_address(),
            publisher.address(),
            publisher.name.clone(),
        ),
        codebase: "https://github.com/JamiiDao/SolanaPayments".to_string(),
        website: Some("https://jamiidao.app".to_string()),
        package_uri: "https://crates.io/crates/solana-payments".to_string(),
        docs: "https://docs.rs/solana-payments/".to_string(),
        phishing: vec![phishing2],
        category: ProjectCategory::Payments,
        language: CodeLanguage::Rust,
        description: "A lightweight library for parsing and creating Solana Pay URLs written in Rust.".to_string()
    };

    vec![project1,project2]
}
     */
