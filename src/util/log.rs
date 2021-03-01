use bincode;

pub struct Log {
    pub path: String,
}

impl Log {
    pub fn install_package(&self, package: &String) {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        let _ = db.insert(bincode::serialize(package).unwrap(), bincode::serialize(&true).unwrap());
        let _ = db.flush();
    }

    pub fn has_package(&self, package: &String) -> bool {
        let db: sled::Db = sled::open(self.path.as_str()).unwrap();

        let has = db.contains_key(bincode::serialize(package).unwrap()).unwrap();

        let _ = db.flush();

        return has;
    }

    pub fn is_installed(&self, package: &String) -> bool {
        if self.has_package(package) {
            let db: sled::Db = sled::open(self.path.as_str()).unwrap();
            let value = db.get(bincode::serialize(package).unwrap()).unwrap();

            let result = bincode::deserialize(&value.unwrap()).unwrap();
            return result;
        }
        else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_install() {
        let log = super::Log {
            path: String::from("tests/files/test_install.db"),
        };

        log.install_package(&String::from("rvpkg"));
        
        assert_eq!(true, log.is_installed(&String::from("rvpkg")));
        assert_ne!(true, log.is_installed(&String::from("rustc")));
    }
    
}