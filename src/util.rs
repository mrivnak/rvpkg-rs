pub mod io {
    pub fn get_log() -> Vec<String> {
        let contents = std::fs::read_to_string("/var/lib/rvpkg/packages.log").expect("Unable to read package log");
        let data: Vec<String> = contents.lines().map(String::from).collect();

        return data;
    }
}

pub mod pkg {
    pub fn get_stripped_log(log: &[String]) -> Vec<String> {

    }

    pub fn parse_packages(in_pkgs: &mut [&str]) -> Vec<super::data::Package> {

    }
}

pub mod data {
    pub struct Package {
        name: String,
        installed: bool,
        req_deps: Vec<String>,
        rec_deps: Vec<String>,
        opt_deps: Vec<String>,
        req_run_deps: Vec<String>,
        rec_run_deps: Vec<String>,
        opt_run_deps: Vec<String>
    }
}