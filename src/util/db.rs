use bincode;

pub struct DB {
    pub path: String,
}

impl DB {
    pub fn get_package(&self, package: &String) -> Result<super::data::Package, String> {
        let raw = self.get_raw(package);

        // TODO: get data from sled db and insert into struct
        match raw {
            Ok(deps) => {
                return Ok(super::data::Package {
                    name: package.clone(),
                    installed: false,
                    dependencies: deps.split(";").map(|s| s.to_string()).collect(),
                });
            },
            Err(e) => return Err(e),
        }
    }

    fn get_raw(&self, package: &String) -> Result<String, String> {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();
        let value = db.get(package);

        match value {
            Ok(v) => {
                return Ok(bincode::deserialize(&v.unwrap()).unwrap());
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn has_package(&self, package: &String) -> bool {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        let has = db.contains_key(package).unwrap();

        let _ = db.flush();

        return has;
    }

    pub fn new_package(&self, package: super::data::Package) {
        self.add_raw(&package.name, &package.dep_string());
    }

    fn add_raw(&self, name: &String, deps: &String) {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        let _ = db.insert(name.as_str(), bincode::serialize(deps).unwrap());
        let _ = db.flush();
    }

    pub fn import_csv(&self, path: &String, mode: bool) {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        if mode {
            self.empty_db();
        }

        let log = super::io::Log {
            path: super::paths::get_log_path(),
        };

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

    fn empty_db(&self) {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        let _ = db.clear();

        let _ = db.flush();
    }

    fn get_size(&self) -> usize {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        return db.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_empty_db() {
        let db = super::DB {
            path: String::from("tests/files/test_add_empty_db.db"),
        };

        db.empty_db();

        db.add_raw(&String::from("rvpkg"), &String::from("rustc;"));

        assert_eq!(1, db.get_size());

        db.empty_db();

        assert_eq!(0, db.get_size());
    }

    #[test]
    fn test_add_get() {
        let db = super::DB {
            path: String::from("tests/files/test_add_get.db"),
        };

        let val_in = String::from("test");

        db.add_raw(&String::from("key"), &val_in);

        let val_out = db.get_raw(&String::from("key")).unwrap();

        assert_eq!(val_in, val_out);        
    }

    #[test]
    fn test_add_get_pkg() {
        let db = super::DB {
            path: String::from("tests/files/test_add_get_pkg.db"),
        };

        let val_in = String::from("rustc;");

        db.add_raw(&String::from("rvpkg"), &val_in);

        let val_out = db.get_package(&String::from("rvpkg")).unwrap();

        assert_eq!(String::from("rvpkg"), val_out.name);
        assert_eq!(String::from("rustc"), val_out.dependencies[0]);
    }
}