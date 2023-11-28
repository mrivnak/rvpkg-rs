use std::error::Error;
use std::fs;
use std::path::Path;
use crate::package::Package;

mod package;

pub fn read_package(path: &Path) -> Result<Package, Box<dyn Error>> {
    let toml_str = fs::read_to_string(path)?;
    let package: Package = toml::from_str(toml_str.as_str())?;
    validate(&package)?;
    Ok(package)
}

fn validate(package: &Package) -> Result<(), anyhow::Error> {
    // check various non-empty vectors
    if package.definition.provides.is_empty() {
        return Err(anyhow::anyhow!("package definition 'provides' cannot be empty"));
    }
    if package.definition.architectures.is_empty() {
        return Err(anyhow::anyhow!("package definition 'architectures' cannot be empty"));
    }

    // Must be 'any' or a list of valid architectures
    if package.definition.architectures.len() > 1 && package.definition.architectures.contains(&"any".to_string()) {
        return Err(anyhow::anyhow!("package definition architecture cannot contain 'any' and other values"));
    }

    Ok(())
}
