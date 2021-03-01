pub mod cli;
pub mod data;
pub mod db;
pub mod io;
pub mod pkg;
pub mod paths;
pub mod log;

pub fn is_pos_int(s: String) -> Result<(), String> {
    let test = s.parse::<u64>().is_ok();

    return if test { return Ok(()); } else { Err(String::from("Value must be a positive integer")) };
}

pub fn file_exists(s: String) -> Result<(), String> {
    return if std::path::Path::new(s.as_str()).exists() { return Ok(()); } else { Err(String::from("Error: file not found!")) };
}

pub fn print_pkg_table(pkgs: &[data::Package]) {

}