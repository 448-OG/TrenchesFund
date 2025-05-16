use dioxus::prelude::*;
use trenchesfund_common::{Outcome, Project, REST_ENDPOINT};
use wallet_adapter::web_sys;

use crate::{
    CodebaseSvg, DocsSvg, InternetSvg, Loader, NotificationInfo, PackageSvg, PhishingSvg, Route,
    SolanaFoundationLogoSvg, GLOBAL_MESSAGE,
};

#[component]
pub fn ProjectView(id: Vec<String>) -> Element {
    let mut project_info = use_signal(|| Option::<Project>::None);

    let send_err_notify = move |msg: &str| {
        GLOBAL_MESSAGE
            .write()
            .push_back(NotificationInfo::error(msg));
        project_info.clone().write().take();
    };

    let id = id.first().cloned();

    use_effect(move || {
        if let Some(id_valid) = id.as_ref().cloned() {
            spawn(async move {
                let endpoint = String::new() + REST_ENDPOINT + "/project-info/" + id_valid.as_str();

                if let Ok(response) = reqwest::Client::new().post(&endpoint).send().await {
                    let body = if let Ok(text) = response.text().await {
                        text
                    } else {
                        send_err_notify("Unable to get the body text");
                        return;
                    };

                    if let Ok(data) = serde_json::from_str::<Outcome<Option<Project>>>(&body) {
                        match data {
                            Outcome::Success(success_data) => {
                                project_info.write().replace(success_data.unwrap());
                            }
                            Outcome::Failure(failure_data) => {
                                send_err_notify(&failure_data);
                            }
                        }
                    } else {
                        send_err_notify(
                            "Unable to fetch projects! Check your connection and try again",
                        );
                        project_info.write().take();
                    }
                } else {
                    send_err_notify(
                        "Unable to fetch projects! Check your connection and try again",
                    );
                    project_info.write().take();
                }
            });
        } else {
            web_sys::console::log_1(&"INVALID ID".into());
            send_err_notify("The route received an invalid id");
        }
    });

    rsx! {
        if let Some(project) = project_info.read().as_ref() {
                div { class: "flex flex-col justify-center items-center w-full",
                    div { class: "w-full flex justify-center items-center",
                        div { class: "flex flex-col justify-center items-center w-[30%]",
                            div { class: "flex flex-col w-full p-5 items-center justify-center",
                                div { class: "w-[200px] rounded-xl mt-10 p-2 shadow-soft-dark",
                                    img {
                                        class: "w-full h-auto",
                                        src: project.icon.as_str(),
                                    }
                                }
                                div { class: "flex text-wrap mt-8",
                                    h1 { class: "text-4xl text-wrap flex flex-col break-all", {project.name_short()}}
                                }
                                div { class: "w-[60%] justify-center mt-5 gap-4 flex",
                                    div { class: "bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                                        {project.language.to_string()}
                                    }
                                    div { class: "bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                                        {project.category.to_string()}
                                    }
                                }
                                div { class: "flex items-center justify-center mt-10 w-full",
                                    Link {
                                        class: "bg-true-blue hover:bg-blue-900 rounded-full py-2 px-4",
                                        to: Route::OrganizationView{id: vec![project.publisher_address().to_string()]},
                                        "View Publisher"
                                    }
                                }
                            }
                        }
                        div { class: "flex-col w-[50%] content-evenly p-5 mt-10",
                            div { class: "w-[80%] flex",
                                h4 { class: "text-blue-200 text-sm", {project.description.as_str()} }
                            }
                            div { class: "flex w-[90%] flex-wrap gap-8 mt-5 items-start justify-start",
                                div { class: "flex",
                                    div { class: "flex w-[20px] mr-2", {CodebaseSvg()} }
                                    div { class: "flex",
                                        a {
                                            class: "text-blue-200",
                                            href: project.codebase.as_str(),
                                            "Codebase⇗"
                                        }
                                    }
                                }
                                if let Some(website) = project.website.as_ref() {
                                    div { class: "flex",
                                        div { class: "flex w-[20px] mr-2", {InternetSvg()}}
                                        div { class: "flex",
                                            a {
                                                class: "text-blue-200",
                                                href: website.as_str(),
                                                "Website⇗"
                                            }
                                        }
                                    }
                                }
                                div { class: "flex",
                                    div { class: "flex w-[20px] mr-2", {PackageSvg()} }
                                    div { class: "flex",
                                        a { class: "text-blue-200", href: project.package_uri.as_str(), "Package⇗" }
                                    }
                                }
                                div { class: "flex",
                                    div { class: "flex w-[20px] mr-2", {DocsSvg()} }
                                    div { class: "flex",
                                        a { class: "text-blue-200", href: project.docs.as_str(), "Docs⇗" }
                                    }
                                }
                            }
                            div { class: "flex w-full gap-8 mt-10 items-center justify-start",
                                span { class: "flex", "Supported By:" }
                                span { class: "flex w-[200px]",
                                    {SolanaFoundationLogoSvg()}
                                }
                            }
                        }
                    }
                }
                hr { class: "flex border-[0.1px] border-true-blue opacity-0 w-[60%] mt-20" }
                div { class: "flex flex-col w-[90%] mt-5",
                    div { class: "flex flex-col w-full justify-items-center justify-center items-center mb-5",
                        div { class: "flex w-full justify-items-center justify-center items-center",
                            span { class: "flex items-center justify-center w-[50px] mr-2",
                                {PhishingSvg()}
                            }
                            span { class: "flex",
                                h1 { class: "flex items-center justify-center text-center text-wrap text-3xl text-blue-200 w-full",
                                    "Phishing Links"
                                }
                            }
                       }
                        div { class: "flex w-full justify-items-center justify-center items-center mt-5",
                            "A list of projects that mislead users by providing phishing websites, codebases, social media handles and meme coins."
                        }

                    }
                    div { class: "flex flex-wrap w-full items-center justify-start mb-20",
                        div { class: "mt-10 flex-col w-full justify-enter items-center flex-wrap p-4",
                            for phishing in project.phishing.as_slice() {
                                div { class: "flex flex-col items-start p-5 w-[90%] border-true-blue border-b-[1px] border-t-[1px] border-opacity-15",
                                    div { class: "flex flex-col w-full",
                                        div { class: "flex w-full items-start flex-col",
                                            h1 { class: "text-center text-2xl", {phishing.name.as_str()} }
                                            div { class: "text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between",
                                                h1 { class: "flex", {phishing.analysis.as_str()}}
                                            }
                                            div { class: "flex mt-10 w-full items-center gap-8 justify-start",
                                                for (name, uri) in phishing.other_uri.as_slice() {
                                                    div { class: "text-black text-lg dark:text-white mt-2 flex items-start justify-between",
                                                        div { class: "flex items-center",
                                                            span { class: "w-[20px]", {InternetSvg()}}
                                                            span { class: "flex text-sm pl-2",
                                                                a {
                                                                    class: "underline",
                                                                    href: uri.as_str(),
                                                                    rel: "noopener noreferrer",
                                                                    target: "_blank",
                                                                    {name.as_str()}"⇗"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        else {
            {Loader()}
        }
    }
}
