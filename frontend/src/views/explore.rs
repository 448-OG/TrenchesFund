use atoll_common::Project;
use dioxus::prelude::*;

use crate::{InstitutionIcon, Route};

#[component]
pub fn Explore() -> Element {
    let projects = vec![Project::default()];

    rsx! {
        div {id:"explore-top", class: "flex-col justify-around items-center w-full min-h-[100vh] p-5",
            for project in projects {

                Link {
                    class: "flex-col w-full h-full items-center justify-center p-4 gap-8",
                     to: Route::ProjectView{ id: vec![project.publisher_address().to_string()]},
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
                    div { class: "mt-10 flex-col w-full justify-start items-start flex-wrap p-4",
                        div { class: "flex rounded-lg flex-col items-start p-5 w-[300px] hover:bg-blue-700 bg-true-blue",
                            div { class: "flex items-center",
                                span { class: "w-[100px] pr-2",
                                    img {class:"w-full h-auto  rounded-xl", alt: project.name.clone()+" icon", src: project.icon, }
                                }
                                 div { class: "w-full justify-start gap-4 items-start flex",
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
                                                {project.publisher_shortened_address()}
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
