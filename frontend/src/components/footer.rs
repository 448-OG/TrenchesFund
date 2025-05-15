use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "flex p-3 items-center justify-center w-full ",
            div{
                p { class: "flex flex-wrap md:flex-row lg:flex-row items-center justify-center", "🄯2025 Atoll AGPLv3 LICENSE"}
            }
        }
    }
}
