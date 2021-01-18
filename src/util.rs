pub mod cli {
    pub fn print_packages(packages: &[super::data::Package]) {
        
    }
}

pub mod data {
    #[derive(Clone)]  // Implement clone trait for package
    pub struct Package {
        pub name: String,
        pub installed: bool,
        pub req_deps: Vec<String>,
        pub rec_deps: Vec<String>,
        pub opt_deps: Vec<String>,
        pub req_run_deps: Vec<String>,
        pub rec_run_deps: Vec<String>,
        pub opt_run_deps: Vec<String>
    }
}

pub mod io {
    pub fn get_log() -> Vec<String> {
        let contents = std::fs::read_to_string("fs/var/lib/rvpkg/packages.log").expect("Unable to read package log");

        if contents == "Unable to read package db" {
            eprintln!("Error: Unable to read package log");
            std::process::exit(1);
        }

        let data: Vec<String> = contents.lines().map(String::from).collect();

        return data;
    }

    pub fn get_db() -> String {
        let contents = std::fs::read_to_string("fs/var/lib/rvpkg/packages.log").expect("Unable to read package db");

        if contents == "Unable to read package db" {
            eprintln!("Error: Unable to read package db");
            std::process::exit(1);
        }

        return contents
    }
}

pub mod pkg {
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

    pub fn parse_packages(in_pkgs: &[String]) -> Vec<super::data::Package> {
        let mut out_pkgs: Vec<super::data::Package> = Vec::new();

        for pkg in in_pkgs {
            let mut meta_pkg = pkg.clone();

            if super::db::has_package(meta_pkg) {
                out_pkgs.push(super::db::get_package(meta_pkg));
            }
            else {
                eprintln!("Error: Package \"{}\" not found in database, exiting...", meta_pkg);
                std::process::exit(1);
            }
        }

        return out_pkgs;
    }
}

mod db {
    pub fn get_package(package: String) -> super::data::Package {
        // TODO: returns a package struct for the specified package

        // TODO: get data from sled db and insert into struct
    }

    pub fn has_package(package: String) -> bool {
        // TODO: query db for a package matching the specified name
        return false;
    }

    pub fn new_package(package: super::data::Package) {
        // TODO: add package to database
    }
}