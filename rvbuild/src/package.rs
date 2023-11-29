use http::Uri;
use nonempty::NonEmpty;
use semver::Version;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Package {
    pub definition: PackageDefinition,
    pub build: PackageBuild,
}

#[derive(Deserialize)]
pub struct PackageDefinition {
    pub name: String,
    pub version: Version,
    pub revision: u32,
    pub description: String,
    pub license: String,
    pub homepage: String,
    pub dependencies: Vec<String>,
    pub build_dependencies: Vec<String>,
    pub provides: NonEmpty<String>,
    pub architectures: NonEmpty<PackageArchitecture>,
}

#[derive(Deserialize)]
pub struct PackageBuild {
    pub source: HashMap<String, PackageSource>,
    pub prepare: Option<String>,
    pub build: String,
    pub install: String,
    pub post_install: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum PackageSource {
    #[serde(rename = "git")]
    Git {
        #[serde(with = "http_serde::uri")]
        url: Uri,
        branch: Option<String>,
        tag: Option<String>,
        commit: Option<String>,
    },
    #[serde(rename = "archive")]
    Archive {
        #[serde(with = "http_serde::uri")]
        url: Uri,
        sha256: String,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PackageArchitecture {
    Any,
    X86,
    X86_64,
    Arm,
    Aarch64,
    Mips,
    Mips64,
    Powerpc,
    Powerpc64,
    Riscv64,
    S390x,
    Sparc64,
}
