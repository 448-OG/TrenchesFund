use dioxus::prelude::*;
use wallet_adapter::{
    web_sys::{self, Window},
    Cluster,
};

pub(crate) static WINDOW: GlobalSignal<Window> =
    Signal::global(|| web_sys::window().expect("Unable to find Window"));

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct AdapterCluster {
    name: String,
    cluster: Cluster,
    endpoint: String,
}

impl AdapterCluster {
    pub fn devnet() -> Self {
        AdapterCluster {
            name: "devnet".to_string(),
            cluster: Cluster::DevNet,
            endpoint: Cluster::DevNet.endpoint().to_string(),
        }
    }
}

impl Default for AdapterCluster {
    fn default() -> Self {
        Self::devnet()
    }
}

impl std::fmt::Display for AdapterCluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cluster.display())
    }
}
