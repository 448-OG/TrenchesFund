use dioxus::prelude::*;

use crate::Loader;

#[component]
pub fn OrganizationView(id: Vec<String>) -> Element {
    rsx! {
        {Loader()}
    }
}
