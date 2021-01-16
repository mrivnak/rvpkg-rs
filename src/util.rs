pub mod io {
    pub fn get_log() -> Vec<String> {
        let contents = std::fs::read_to_string("/var/lib/rvpkg/packages.log").expect("Unable to read package log");
        let data: Vec<String> = contents.lines().map(String::from).collect();

        return data;
    }

    pub fn get_db() -> Vec<String> {
        let contents = std::fs::read_to_string("/var/lib/rvpkg/packages.log").expect("Unable to read package log");
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

    pub fn parse_packages(in_pkgs: &mut [String]) -> Vec<super::data::Package> {
        let mut out_pkgs: Vec<super::data::Package> = Vec::new();
        let log: Vec<String> = super::io::get_log().to_vec();
        let package_data = super::yaml::get_package_data();
        let package_names = super::yaml::get_package_names();

        for pkg in in_pkgs {
            // TODO: add handling for roughly matching package names
            let mut matches: Vec<String> = Vec::new();
            for name in package_names {
                if name.contains(pkg.as_str()) {
                    matches.push(name);
                }
            }

            match matches.len() {
                0 => {}  // TODO: exit
                1 => {}  // Continue
                _ => {
                    loop {
                        println!("Package \"{}\" has multiple matches...", pkg);
                        for (i, n) in matches.iter().enumerate() {
                            println!("{} {}", i + 1, pkg);
                        }

                        let index: String = text_io::read!("{}\n");

                        if index.parse::<u16>().is_ok() &&
                            index.parse::<u16>().unwrap() > 0 &&
                            index.parse::<u16>().unwrap() < matches.len() as u16 {
                            let index = index.parse::<usize>().unwrap();
                            let pkg = matches.get(index);
                            break;
                        }
                        else {
                            // TODO: error out and try again
                            continue;
                        }
                    }
                }
            }

            match package_data.get(pkg) {
                Some(&package) => {
                    out_pkgs.push(super::data::Package {
                        installed: log.contains(&package.name),
                        ..package
                    })
                }
                _ => unreachable!()
            }
        }

        return out_pkgs;
    }
}

pub mod data {
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

    pub fn get_package_data() -> HashMap<String, super::data::Package> {
        // TODO: return a map of package names and data
    }

    pub fn get_package_names() -> Vec<String> {
        // TODO: return a list of package names
    }

    pub fn new_package(package: super::data::Package) {
        // TODO: add package to database
        // should provide formatted lines to super::io::write_lines()
    }
}