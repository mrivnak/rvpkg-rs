pub mod io {
    pub fn get_log() -> Vec<String> {
        let contents = std::fs::read_to_string("/var/lib/rvpkg/packages.log").expect("Unable to read package log");

        if contents == "Unable to read package db" {
            eprintln!("Error: Unable to read package log");
            std::process::exit(1);
        }

        let data: Vec<String> = contents.lines().map(String::from).collect();

        return data;
    }

    pub fn get_db() -> String {
        let contents = std::fs::read_to_string("/var/lib/rvpkg/packages.log").expect("Unable to read package db");

        if contents == "Unable to read package db" {
            eprintln!("Error: Unable to read package db");
            std::process::exit(1);
        }

        return contents
    }

    pub fn get_db_lines() -> Vec<String> {
        let contents = std::fs::read_to_string("/usr/share/rvpkg/packages.yaml").expect("Unable to read package db");

        if contents == "Unable to read package db" {
            eprintln!("Error: Unable to read package db");
            std::process::exit(1);
        }

        let data: Vec<String> = contents.lines().map(String::from).collect();

        return data;
    }

    pub fn write_lines(lines: Vec<String>) {
        // TODO: write lines to database file
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
        let log: Vec<String> = super::io::get_log().to_vec();
        let package_data = super::yaml::get_package_data();
        let package_names = super::yaml::get_package_names();

        for pkg in in_pkgs {
            let mut meta_pkg = pkg.clone();
            let mut matches: Vec<String> = Vec::new();
            for name in &package_names {
                if name.contains(pkg.as_str()) {  // Find a list of matching package names
                    matches.push(name.to_string());
                }
            }

            match matches.len() {
                0 => {
                    eprintln!("Error: Package \"{}\" not found in database. Exiting...", meta_pkg);
                    std::process::exit(1);
                }
                1 => {}  // Continue
                _ => {
                    loop {
                        println!("Package \"{}\" has multiple matches...", meta_pkg);
                        for (i, n) in matches.iter().enumerate() {
                            println!("{} {}", i + 1, n);  // Print matching package names
                        }

                        print!("Package # to select: ");
                        let index: String = text_io::read!("{}\n");

                        if index.parse::<u16>().is_ok() &&
                                index.parse::<u16>().unwrap() > 0 &&
                                index.parse::<u16>().unwrap() < matches.len() as u16 {  // Verify that the input is a u16 and is within the range of matches

                            let index = index.parse::<usize>().unwrap();
                            meta_pkg = matches.get(index)
                                .expect(format!("Error: Package \"{}\" not found in database. Exiting... (this should be unreachable)", meta_pkg)
                                .as_str()).clone();  // replace 'meta_pkg' with the selected package name
                            break;
                        }
                        else {
                            eprintln!("Error: Invalid selection");
                            continue;
                        }
                    }
                }
            }

            match package_data.get(&meta_pkg) {  // get package from the database hashmap
                Some(package) => {
                    let new_package: super::data::Package = package.clone();
                    out_pkgs.push(super::data::Package {
                        installed: log.contains(&package.name),
                        ..new_package
                    })  // Update package data with installation status
                }
                _ => unreachable!()  // Already checked if the package is in the hashmap, should be unreachable
            }
        }

        return out_pkgs;
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

mod yaml {
    use std::collections::HashMap;
    use yaml_rust::{YamlEmitter, YamlLoader};

    pub fn get_package_data() -> HashMap<String, super::data::Package> {
        // TODO: return a map of package names and data
        let data = YamlLoader::load_from_str(super::io::get_db().as_str()).unwrap();

        // TODO: translate yaml into  package data
        let out: HashMap<String, super::data::Package> = HashMap::new();
        return out;
    }

    pub fn get_package_names() -> Vec<String> {
        return get_package_data()
            .keys()
            .cloned()
            .collect();
    }

    pub fn new_package(package: super::data::Package) {
        // TODO: add package to database
        // should provide formatted lines to super::io::write_lines()
    }
}