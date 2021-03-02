use super::db;
use super::log;
use super::data::Package;

pub fn get_stripped_log(log: &[String]) -> Vec<String> {
    let mut out = Vec::new();

    // TODO: reimplement by sorting and removing duplicates
    // should be much faster that way
    for package in log {
        if !out.contains(package) {
            out.push(package.to_string());
        }
    }

    out.sort();
    return out;
}

pub fn parse_packages(packages: &[String]) -> Vec<Package> {
    let mut out_pkgs: Vec<Package> = Vec::new();

    let db = db::DB {
        path: super::paths::get_db_path(),
    };

    for pkg in packages {
        // let meta_pkg = pkg.clone();

        if db.has_package(pkg) {
            out_pkgs.push(db.get_package(pkg).unwrap());
        }
        else {
            eprintln!("Error: Package \"{}\" not found in database, exiting...", pkg);
            std::process::exit(1);
        }
    }

    return out_pkgs;
}

pub fn is_installed(package: &String) -> bool {
    let log = log::Log {
        path: super::paths::get_log_path(),
    };

    return log.is_installed(package);
}

pub fn install(package: &String) {
    let log = log::Log {
        path: super::paths::get_log_path(),
    };

    log.install_package(package);
}