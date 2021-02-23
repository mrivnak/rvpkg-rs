use crate::util::io;

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

    for line in io::get_lines(path.as_str()) {
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
