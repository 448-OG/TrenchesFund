use async_dup::Arc;
use async_lock::{OnceCell, RwLock};
use atoll_common::{Outcome, Project, Publisher};
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

#[post("/projects")]
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

#[post("/project-info/<id>")]
async fn projects_info(id: String) -> Json<Outcome<Option<Project>>> {
    match DbState::read(PROJECTS_DB, &id).await {
        Ok(project_exists) => {
            if let Ok(value) = bincode::deserialize::<Project>(&project_exists) {
                Json(Outcome::Success(Some(value)))
            } else {
                Json(Outcome::Failure("Internal Error".to_string()))
            }
        }
        Err(error) => {
            if error == BackendError::KvKeyNotFound {
                Json(Outcome::Success(Option::None))
            } else {
                Json(Outcome::Failure("Internal Server Error".to_string()))
            }
        }
    }
}

#[post("/publisher-info/<id>")]
async fn publisher_info(id: String) -> Json<Outcome<Option<Publisher>>> {
    match DbState::read(PUBLISHERS_DB, &id).await {
        Ok(project_exists) => {
            if let Ok(value) = bincode::deserialize::<Publisher>(&project_exists) {
                Json(Outcome::Success(Some(value)))
            } else {
                Json(Outcome::Failure("Internal Error".to_string()))
            }
        }
        Err(error) => {
            if error == BackendError::KvKeyNotFound {
                Json(Outcome::Success(Option::None))
            } else {
                Json(Outcome::Failure("Internal Server Error".to_string()))
            }
        }
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
            .mount("/", routes![projects, projects_info, publisher_info])
            .launch()
            .await?;

        // let publisher = atoll_common::foo();
        // let projects = atoll_common::projects(&publisher);

        // DbState::create(
        //     PUBLISHERS_DB,
        //     &publisher.address(),
        //     &bincode::serialize(&publisher).unwrap(),
        // )
        // .await
        // .unwrap();

        // for project in projects {
        //     DbState::create(
        //         PROJECTS_DB,
        //         &project.name,
        //         &bincode::serialize(&project).unwrap(),
        //     )
        //     .await
        //     .unwrap();
        // }

        Ok(())
    }
}
