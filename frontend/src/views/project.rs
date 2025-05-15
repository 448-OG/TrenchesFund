use atoll_common::Project;
use dioxus::prelude::*;

use crate::Loader;

#[component]
pub fn ProjectView(id: Vec<String>) -> Element {
    let mut project_info = use_signal(|| Option::<Project>::None);

    rsx! {
        if project_info.read().is_some() {
            {Loader()}
        }else {
            "Some Value"
        }
    }
}
