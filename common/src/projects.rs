use std::fmt::{Debug, Display};

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: blake3::Hash,
    pub name: String,
    pub logo: String,
    pub icon: String,
    pub publisher: (String, String, String), // (Short Address, Address, Publisher Name)
    pub codebase: String,
    pub website: Option<String>,
    pub docs: String,
    pub phishing: Vec<Phishing>,
    pub category: ProjectCategory,
    pub language: CodeLanguage,
    pub package_uri: String,
    pub description: String,
}

impl Project {
    pub fn publisher_shortened_address(&self) -> &str {
        self.publisher.0.as_str()
    }

    pub fn publisher_address(&self) -> &str {
        self.publisher.1.as_str()
    }

    pub fn publisher_name(&self) -> &str {
        self.publisher.2.as_str()
    }

    pub fn name_short(&self) -> String {
        let len = self.name.len();
        let mut name = self.name.clone();

        if len > 25 {
            name.truncate(25);

            name + "..."
        } else {
            name
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        let publisher = Publisher::default();

        Self {
            id: blake3::hash(&[0u8; 32]),
            name: "Bar Project".to_string(),
            logo: "https://picsum.photos/500".to_string(),
            icon: "https://picsum.photos/100".to_string(),
            publisher: (
                publisher.short_address(),
                publisher.address(),
                publisher.name,
            ),
            codebase: "foo://bar.codebase".to_string(),
            website: Option::Some("foo://bar.webzite".to_string()),
            docs: "foo://docs.code.base-docs".to_string(),
            phishing: vec![Phishing::default()],
            category: ProjectCategory::default(),
            language: CodeLanguage::default(),
            package_uri: "foo://packaging-example".to_string(),
            description: "Default example for a project".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Publisher {
    pub name: String,
    pub icon: String,
    pub logo: String,
    pub description: String,
    pub codebase: String,
    pub website: String,
    pub public_key: VerifyingKey,
    pub mint: VerifyingKey,
    pub merch: Vec<Merch>,
}

impl Publisher {
    pub fn address(&self) -> String {
        Self::public_key_to_base58(&self.public_key)
    }

    pub fn short_address(&self) -> String {
        let address = self.address().clone();
        let mut short_address = String::default();

        short_address.push_str(&address[..5]);
        short_address.push_str("...");
        short_address.push_str(&address[address.len() - 4..]);

        short_address
    }

    pub fn mint_address(&self) -> String {
        Self::public_key_to_base58(&self.mint)
    }

    pub fn public_key_to_base58(public_key: &VerifyingKey) -> String {
        let public_key = VerifyingKey::from_bytes(&public_key.to_bytes()).unwrap();

        bs58::encode(&public_key.to_bytes()).into_string()
    }
}

impl Debug for Publisher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Publisher")
            .field("name", &self.name)
            .field("icon", &self.icon)
            .field("logo", &self.logo)
            .field("description", &self.description)
            .field("codebase", &self.codebase)
            .field("website", &self.website)
            .field("public_key", &self.address())
            .field("mint", &self.mint_address())
            .field("merch", &self.merch)
            .finish()
    }
}

impl Default for Publisher {
    fn default() -> Self {
        Self {
            name: "Foo Organization".to_string(),
            icon: "https://picsum.photos/100".to_string(),
            logo: "https://picsum.photos/100".to_string(),
            description: "Default org description".to_string(),
            codebase: "foo://code.base".to_string(),
            website: "foo://example.project".to_string(),
            public_key: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
            mint: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
            merch: vec![Merch::default()],
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Phishing {
    pub name: String,
    pub uri: String,
    pub analysis: String,
    pub other_uri: Vec<(String, String)>,
}

impl Default for Phishing {
    fn default() -> Self {
        Self {
            name: "Foo Organisation".to_string(),
            uri: "foo://foo-org".to_string(),
            analysis: "This is the default example phishing line".to_string(),
            other_uri: vec![(
                "$FOO MEME COIN".to_string(),
                "pump.vun/jhvgjkjhvjfjjhhjfhjkfjjkdhjjjuf".to_string(),
            )],
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Merch {
    pub supplier: String,
    pub image: String,
    pub name: String,
    pub description: String,
}

impl Debug for Merch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Merch")
            .field("supplier", &self.supplier)
            .field("image", &self.image)
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

impl Default for Merch {
    fn default() -> Self {
        Merch {
            supplier: "SolMerch Global Industries".to_string(),
            image: "https://picsum.photos/1280/720".to_string(),
            name: "Random Merch".to_string(),
            description: "Example random merchandise".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum ProjectCategory {
    Cryptography,
    Ai,
    Graphics,
    Text,
    Wallet,
    Payments,
    #[default]
    Unspecified,
}

impl Display for ProjectCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Cryptography => "Cryptography",
                Self::Ai => "AI",
                Self::Wallet => "Wallet",
                Self::Payments => "Payments",
                Self::Graphics => "Graphics",
                Self::Text => "Text",
                Self::Unspecified => "Unspecified",
            }
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum CodeLanguage {
    #[default]
    NonCode,
    Rust,
    TypeScript,
    Go,
    Python,
    JavaScript,
    KotlinJava,
}

impl Display for CodeLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NonCode => "NonCode",
                Self::Rust => "Rust",
                Self::TypeScript => "TypeScript",
                Self::Go => "Go",
                Self::Python => "Python",
                Self::JavaScript => "JavaScript",
                Self::KotlinJava => "KotlinJava",
            }
        )
    }
}
