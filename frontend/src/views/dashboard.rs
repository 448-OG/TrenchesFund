use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "flex flex-col justify-around items-center w-full min-h-[100vh] p-5",
            div {
                "DASHBOARD"
            }
        }
    }
}
