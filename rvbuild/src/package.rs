use http::Uri;
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Package {
    pub definition: PackageDefinition,
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
    pub provides: Vec<String>,
    pub architectures: Vec<String>,
}

#[derive(Deserialize)]
pub struct PackageBuild {
    pub source: PackageSource,
}

#[derive(Deserialize)]
pub struct PackageSource {
    #[serde(with = "http_serde::uri")]
    pub url: Uri,
    pub sha256: String,
}