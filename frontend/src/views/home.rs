use dioxus::prelude::*;

use crate::{Route, LOGO};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col justify-around items-center w-full min-h-[100vh] p-5",
            div { class:"flex flex-col w-full items-center justify-center m-h-[100%] h-full",
                div {class:"flex items-center justify-center w-full", img { class:"flex w-48 sm:w-64", src:LOGO } }
                div{class:"text-6xl font-smooch", "Trenches Fund"}
                div{class:"text-2xl font-smooch text-center ", "Financing Open Source Maintainers while they Build in the Trenches"}

                div{class:"flex p-4 items-center gap-20",
                    Link { class:"bg-true-blue sm:text-6xl md:text-2xl font-smooch rounded-full py-2 px-4 mt-20", to: Route::Explore(), "EXPLORE PROJECTS" }
                    a { class:"bg-true-blue sm:text-6xl md:text-2xl font-smooch rounded-full py-2 px-4 mt-20", href:"#", "START A COMMUNITY" }
                }
            }
        }
    }
}
