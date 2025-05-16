use dioxus::prelude::*;
use trenchesfund_common::{Outcome, Project, REST_ENDPOINT};

use crate::{InstitutionIcon, Loader, NotificationInfo, Route, GLOBAL_MESSAGE};

#[component]
pub fn Explore() -> Element {
    let mut projects_data = use_signal(|| Option::<Vec<Project>>::default());

    let send_err_notify = move |msg: &str| {
        GLOBAL_MESSAGE
            .write()
            .push_back(NotificationInfo::error(msg));
        projects_data.clone().write().replace(Vec::default());
    };

    use_effect(move || {
        spawn(async move {
            let endpoint = String::new() + REST_ENDPOINT + "/projects";

            if let Ok(response) = reqwest::Client::new().post(&endpoint).send().await {
                let body = if let Ok(text) = response.text().await {
                    text
                } else {
                    send_err_notify("Unable to get the body text");
                    return;
                };

                if let Ok(data) = serde_json::from_str::<Outcome<Vec<Project>>>(&body) {
                    match data {
                        Outcome::Success(success_data) => {
                            projects_data.write().replace(success_data);
                        }
                        Outcome::Failure(failure_data) => {
                            send_err_notify(&failure_data);
                        }
                    }
                } else {
                    send_err_notify(
                        "Unable to fetch projects! Check your connection and try again",
                    );
                    projects_data.write().replace(Vec::default());
                }
            } else {
                send_err_notify("Unable to fetch projects! Check your connection and try again");
                projects_data.write().replace(Vec::default());
            }
        });
    });

    rsx! {
        div {id:"explore-top", class: "flex-col justify-around items-center w-full min-h-[100vh] p-5",
            if let Some(projects) = projects_data.read().as_ref() {
                div {
                    class: "flex-col w-full h-full items-center justify-center p-4",

                        div { class: "flex w-full items-center justify-end gap-8",
                            div { class: "flex",
                                select {
                                    class: "flex bg-true-blue rounded-full py-1 px-1 text-center",
                                    id: "language",
                                    name: "type",
                                    option { selected: "false", value: "rs", "Rust" }
                                    option { value: "ts", "TypeScript" }
                                    option { value: "js", "JavaScript" }
                                }
                            }
                            div { class: "flex",
                                select {
                                    class: "flex bg-true-blue rounded-full py-1 px-1 text-center",
                                    id: "language",
                                    name: "type",
                                    option { selected: "false", value: "libraries", "Libraries" }
                                    option { value: "graphics", "Graphics" }
                                }
                            }
                            div { class: "flex",
                                input {
                                    class: "flex bg-transparent border-true-blue border-b-2 py-1 px-1 text-center text-true-blue placeholder:text-blue-yonder outline-none",
                                    id: "language",
                                    name: "type",
                                    placeholder: "search by organization",
                                }
                            }
                        }

                    div { class:"w-full h-full gap-8 items-center justify-start flex flex-wrap",
                        for project in projects.as_slice() {
                                Link { class: "flex rounded-lg flex-col items-start p-5 w-[300px] hover:bg-blue-700 bg-true-blue rounded-xl",
                                to: Route::ProjectView{ id: vec![project.name.to_string()]},
                                div { class:"w-full flex items-center justify-start",
                                        img { class:"bg-white w-[100px] rounded-xl", src:project.icon.as_str() }
                                        div { class: "flex p-2 gap-8 mt-5",
                                                div { class: "flex bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                                                        {project.language.to_string()}
                                                }
                                                div { class: "flex bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                                                        {project.category.to_string()}
                                                }
                                        }
                                    }
                                    div { class: "flex flex-col w-full",
                                        div { class: "flex w-full items-start flex-col mt-2.5",
                                            h5 { class: "text-lg text-wrap flex flex-wrap font-semibold tracking-tight",
                                                {project.name_short()}
                                            }
                                            div { class: "text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between",
                                                div { class: "flex items-center",
                                                    span { class: "w-[20px]",
                                                        {InstitutionIcon()}
                                                    }
                                                    span { class: "flex text-sm pl-2",
                                                        {project.publisher_name()}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                        }
                     }
                }
            }else {
                div { class:"w-full flex h-full items-center justify-center",
                    span{{Loader()}} span{"Fetching projects..."}
                }
            }
        }
    }
}
