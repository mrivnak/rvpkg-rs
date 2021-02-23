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
