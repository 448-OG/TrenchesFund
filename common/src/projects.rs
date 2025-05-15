use std::fmt::{Debug, Display};

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

pub fn foo() -> Publisher {
    let publisher_address = "5bAvxZRsixc9F6fL1Kzv2jfgCZ1UXrVW5FsWcHfppZUy";
    let publisher_bytes: [u8; 32] = bs58::decode(publisher_address)
        .into_vec()
        .unwrap()
        .try_into()
        .unwrap();
    let publisher_pubkey = VerifyingKey::from_bytes(&publisher_bytes).unwrap();

    let merch = vec![
        Merch {
            supplier: "Global SOL Merch Industries".to_string(),
            image: "/assets/sol-mug.jpg".to_string(),
            name: "Solana Branded Mug".to_string(),
            description: "Lovely Solana Branded Mug for sipping while in the trenches".to_string(),
        },
        Merch {
            supplier: "Global SOL Merch Industries".to_string(),
            image: "/assets/sol-hoodie.jpg".to_string(),
            name: "Solana Branded Hoodie".to_string(),
            description: "Stay warm while in the trenches".to_string(),
        },
        Merch {
            supplier: "Laptop Sticker".to_string(),
            image: "/assets/sol-mug.jpg".to_string(),
            name: "Solana Branded Mug".to_string(),
            description: "Laptop powered by Solana love.".to_string(),
        },
    ];

    Publisher {
        name: "JAMII DAO".to_string(),
        public_key: publisher_pubkey,
        mint: VerifyingKey::from_bytes(&[9u8; 32]).unwrap(),
        merch,
    }
}

pub fn projects(publisher: &Publisher) -> Vec<Project> {
    let phishing1 = Phishing { 
        name: "SolanaWalletAdapter".to_string(), 
        uri: "https://github.com/Jamii-Dao/SolanaWalletAdapter".to_string(), 
        analysis: "This project impersonates the publisher of this library. The creator of the organization added a hyphen between `Jamii` and `Dao` to create a Github organization called `Jamii-Dao` which is similar to the publisher of this library `JamiiDao`. They mirrored the commits of this library to make it seem like the publisher of this library is part of their phishing organization.".to_string(), 
        other_uri: vec![
            ("x.com/JamiiDao".to_string(), "x.com/JamiiDao".to_string()),
           ("Phishing Codebase".to_string(), "https://github.com/Jamii-Dao/Solan,aWalletAdapter/".to_string()),
           ("Phishing Website".to_string(), "https://jamiidao.com/".to_string()),
           ("Phishing Meme Coin".to_string(), "https://pump.fun/coin/3ZxaS6sDzdJjiwCRtf3RkWfEw25XeU5bLsvkmkNArxcp".to_string())
        ] 
    };

    let project1_name = "wallet-adapter";
    let project1 = Project {
        id: blake3::hash(project1_name.as_bytes()),
        name: project1_name.to_string(),
        logo: "/assets/wallet-adapter-logo.svg".to_string(),
        icon: "/assets/wallet-adapter-icon.svg".to_string(),
        publisher: (
            publisher.short_address(),
            publisher.address(),
            publisher.name.clone(),
        ),
        codebase: "https://github.com/JamiiDao/SolanaWalletAdapter".to_string(),
        website: Some("https://jamiidao.app".to_string()),
        package_uri: "https://crates.io/crates/wallet-adapter".to_string(),
        docs: "https://jamiidao.github.io/SolanaWalletAdapter/".to_string(),
        phishing: vec![phishing1],
        category: ProjectCategory::Wallet,
        language: CodeLanguage::Rust,
        description: "The wallet-adapter library is a Rust crate that performs actions between a Rust WebAssembly frontend and browser wallet extensions that implement the wallet-standard.".to_string()
    };
    let phishing2 = Phishing { 
        name: " SolanaPayments".to_string(), 
        uri: "https://github.com/Jamii-Dao/SolanaPayments".to_string(), 
        analysis: "This project impersonates the publisher of this library. The creator of the organization added a hyphen between `Jamii` and `Dao` to create a Github organization called `Jamii-Dao` which is similar to the publisher of this library `JamiiDao`. They mirrored the commits of this library to make it seem like the publisher of this library is part of their phishing organization.".to_string(), 
        other_uri: vec![
            ("x.com/JamiiDao".to_string(), "x.com/JamiiDao".to_string()),
           ("Phishing Codebase".to_string(), "https://github.com/Jamii-Dao/SolanaPayments".to_string()),
           ("Phishing Website".to_string(), "https://jamiidao.com/".to_string()),
           ("Phishing Meme Coin".to_string(), "https://pump.fun/coin/3ZxaS6sDzdJjiwCRtf3RkWfEw25XeU5bLsvkmkNArxcp".to_string())
        ] 
    };

    let project2_name = "solana-payments";
    let project2 = Project {
        id: blake3::hash(project2_name.as_bytes()),
        name: project2_name.to_string(),
        logo: "/assets/solana-payments-logo.svg".to_string(),
        icon: "/assets/solana-payments-icon.svg".to_string(),
        publisher: (
            publisher.short_address(),
            publisher.address(),
            publisher.name.clone(),
        ),
        codebase: "https://github.com/JamiiDao/SolanaPayments".to_string(),
        website: Some("https://jamiidao.app".to_string()),
        package_uri: "https://crates.io/crates/solana-payments".to_string(),
        docs: "https://docs.rs/solana-payments/".to_string(),
        phishing: vec![phishing2],
        category: ProjectCategory::Payments,
        language: CodeLanguage::Rust,
        description: "A lightweight library for parsing and creating Solana Pay URLs written in Rust.".to_string()
    };

    vec![project1,project2]
}

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
    pub description: String
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
            description: "Default example for a project".to_string()
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Publisher {
    pub name: String,
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
