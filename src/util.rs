pub mod cli {
    pub fn print_packages(packages: &[super::data::Package]) {
        
    }
}

pub mod data {
    #[derive(Clone, Debug)]  // Implement clone trait for package
    pub struct Package {
        pub name: String,
        pub installed: bool,
        pub dependencies: Vec<String>
    }

    impl From::<Package> for String {
        fn from(package: Package) -> String {
            return package.name;
        }
    }

    impl From::<&Package> for String {
        fn from(package: &Package) -> String {
            let meta_package = package.clone();
            return meta_package.name;
        }
    }

    impl Package {
        pub fn dep_string(&self) -> String {
            let mut s: String = String::from("");

            for dep in self.dependencies.iter() {
                s.push_str(dep.as_str());
                s.push_str("; ");
            }

            s.trim_end();

            return s;
        }
    }
}

pub mod io {
    pub fn get_lines(path: &str) -> Vec<String> {
        let contents = std::fs::read_to_string(path).expect("Unable to read package log");

        if contents == "Unable to read package db" {
            eprintln!("Error: Unable to read package log");
            std::process::exit(1);
        }

        let data: Vec<String> = contents.lines().map(String::from).collect();

        return data;
    }
    pub fn get_log() -> Vec<String> {
        return get_lines("fs/var/lib/rvpkg/packages.log");
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
            let meta_pkg = pkg.clone();

            if super::db::has_package(&meta_pkg) {
                out_pkgs.push(super::db::get_package(&meta_pkg).unwrap());
            }
            else {
                eprintln!("Error: Package \"{}\" not found in database, exiting...", meta_pkg);
                std::process::exit(1);
            }
        }

        return out_pkgs;
    }
}

pub mod db {
    pub fn get_package(package: &String) -> Result<super::data::Package, String> {
        // TODO: returns a package struct for the specified package
        if !has_package(package) {
            return Err(String::from("Package not in database"));
        }

        let db: sled::Db = sled::open("fs/usr/share/rvpkg/packages.db").unwrap();

        // TODO: get data from sled db and insert into struct
        return Ok(super::data::Package {
            name: String::from(""),
            installed: false,
            dependencies: Vec::new()
        });
    }

    pub fn has_package(package: &String) -> bool {
        let db: sled::Db = sled::open("fs/usr/share/rvpkg/packages.db").unwrap();

        let has = db.contains_key(package).unwrap();

        let _ = db.flush();

        return has;
    }

    pub fn new_package(package: super::data::Package) {
        // TODO: add package to database
        let db: sled::Db = sled::open("fs/usr/share/rvpkg/packages.db").unwrap();

        let _ = db.insert(&package.name, package.dep_string().as_str());
        let _ = db.flush();
    }

    pub fn add_raw(name: &String, deps: &String) {
        let db: sled::Db = sled::open("fs/usr/share/rvpkg/packages.db").unwrap();

        let _ = db.insert(name.as_str(), deps.as_str());
        let _ = db.flush();
    }

    pub fn import_csv(path: &String, mode: bool) {
        let db: sled::Db = sled::open("fs/usr/share/rvpkg/packages.db").unwrap();

        if mode {
            let _ = db.clear();
        }

        for line in super::io::get_lines(path.as_str()) {
            let items: Vec<&str> = line.split_terminator(",").collect();
            if items.len() != 2 {
                eprintln!("Error: invalid line in csv, ignoring...");
                eprintln!("Error: line: {}", line);
            }
            else {
                let package = items[0];
                let deps = items[1];

                let _ = db.insert(package, deps);
            }
        }

        let _ = db.flush();
    }
}