use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::package::{Package, PackageArchitecture};

pub mod package;
mod build;
mod command;

pub fn read_package(path: &Path) -> Result<Package, Box<dyn Error>> {
    let toml_str = fs::read_to_string(path)?;
    let package: Package = toml::from_str(toml_str.as_str())?;
    validate(&package)?;
    Ok(package)
}

fn validate(package: &Package) -> Result<(), anyhow::Error> {
    // Must be 'any' or a list of valid architectures
    if package.definition.architectures.len() > 1 && package.definition.architectures.iter().any(|a| matches!(a, PackageArchitecture::Any)) {
        return Err(anyhow::anyhow!("package definition architecture cannot contain 'any' and other values"));
    }

    Ok(())
}

pub fn insert_variables(package: &mut Package) {
    let outdir = format!("/var/tmp/rvbuild/{}", package.definition.name);
    let variables = HashMap::from([
        ("$OUTDIR", outdir.as_str())
    ]);

    for (key, value) in variables {
        match package.build.prepare {
            Some(ref mut prepare) => *prepare = prepare.replace(key, value),
            None => {}
        }
        package.build.build = package.build.build.replace(key, value);
        package.build.install = package.build.install.replace(key, value);
        match package.build.post_install {
            Some(ref mut post_install) => *post_install = post_install.replace(key, value),
            None => {}
        }
    }
}
