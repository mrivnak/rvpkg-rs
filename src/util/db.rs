use bincode;

pub struct DB {
    pub path: String,
}

impl DB {
    pub fn get_package(&self, package: &String) -> Result<super::data::Package, String> {
        let raw = self.get_raw(package);

        match raw {
            Ok(deps) => {
                return Ok(super::data::Package {
                    name: package.clone(),
                    installed: false,
                    dependencies: deps.split_terminator(";").map(|s| s.to_string()).collect(),
                });
            }
            Err(e) => return Err(e),
        }
    }

    fn get_raw(&self, package: &String) -> Result<String, String> {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();
        let value = db.get(bincode::serialize(package).unwrap());

        match value {
            Ok(v) => {
                return Ok(bincode::deserialize(&v.unwrap()).unwrap());
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn find_key(&self, search: &String) -> Vec<String> {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        let mut out: Vec<String> = Vec::new();
        for item in db.iter() {
            let key: String = bincode::deserialize(&item.unwrap().0).unwrap();
            if key.contains(search) {
                out.push(key);
            }
        }

        return out;
    }

    pub fn has_package(&self, package: &String) -> bool {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        let has = db
            .contains_key(bincode::serialize(package).unwrap())
            .unwrap();

        let _ = db.flush();

        return has;
    }

    pub fn add_raw(&self, name: &String, deps: &String) {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        let _ = db.insert(
            bincode::serialize(name).unwrap(),
            bincode::serialize(deps).unwrap(),
        );
        let _ = db.flush();
    }

    pub fn import_csv(&self, path: &String, mode: bool) {
        if mode {
            self.empty_db();
        }

        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        for line in super::io::get_lines(path.as_str()).unwrap() {
            let items: Vec<&str> = line.split_terminator(",").collect();
            if items.len() > 2 {
                eprintln!("Error: invalid line in csv, ignoring...");
                eprintln!("Error: line: {}", line);
            }
            if items.len() == 1 {
                let package = items[0];

                let _ = db.insert(
                    bincode::serialize(package).unwrap(),
                    bincode::serialize("").unwrap(),
                );
            } else {
                let package = items[0];
                let deps = items[1];

                let _ = db.insert(
                    bincode::serialize(package).unwrap(),
                    bincode::serialize(deps).unwrap(),
                );
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

        return db.len();
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
        assert!(db.has_package(&String::from("rvpkg")));

        let val_out = db.get_package(&String::from("rvpkg")).unwrap();

        assert_eq!(String::from("rvpkg"), val_out.name);
        assert_eq!(String::from("rustc"), val_out.dependencies[0]);
    }

    #[test]
    fn test_import_csv() {
        let db = super::DB {
            path: String::from("tests/files/test_import_csv.db"),
        };

        db.import_csv(&String::from("tests/files/test_import.csv"), true);

        let val_out = db.get_package(&String::from("rvpkg")).unwrap();

        assert_eq!(String::from("rvpkg"), val_out.name);
        assert_eq!(String::from("rustc"), val_out.dependencies[0]);

        let val_out = db.get_package(&String::from("clang")).unwrap();

        assert_eq!(String::from("clang"), val_out.name);
        assert_eq!(String::from("llvm"), val_out.dependencies[0]);
    }
}
