use serde::Deserialize;

#[derive(Clone, Debug)]  // Implement clone trait for package
pub struct Package {
    pub name: String,
    pub installed: bool,
    pub dependencies: Vec<String>
}

impl Package {
    pub fn dep_string(&self) -> String {
        let mut s: String = String::from("");

        for dep in self.dependencies.iter() {
            s.push_str(dep.as_str());
            s.push_str("; ");
        }

        s = s.trim_end().to_string();

        return s;
    }
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

#[derive(Deserialize)]
pub struct Settings {
    pub verbose:    bool,
    pub no_confirm: bool,
    pub show_deps:  bool,
    pub color:      bool,
}
