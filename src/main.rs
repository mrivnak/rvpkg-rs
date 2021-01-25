use clap::{Arg, App, AppSettings, SubCommand};

mod util;

struct Settings {
    verbose:    bool,
    no_confirm: bool,
    runtime:    bool,
    show_deps:  bool,
    color:      bool
}

fn main() {
    // Get information from Cargo.toml
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION"); 

    // Get settings from rvpkg.toml

    // Argument parsing
    let app = App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .author(AUTHORS)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Display verbose output")
            .global(true))
        .arg(Arg::with_name("no-confirm")
            .short("y")
            .long("no-confirm")
            .help("Accept package updates without prompting the user")
            .global(true))
        .arg(Arg::with_name("runtime")
            .short("r")
            .long("runtime")
            .help("Display runtime dependencies")
            .global(true))
        .arg(Arg::with_name("show-deps")
            .short("d")
            .long("show-deps")
            .help("Display package dependencies")
            .global(true))
        .arg(Arg::with_name("color")
            .short("c")
            .long("color")
            .help("Display colored output")
            .global(true))
        .subcommand(SubCommand::with_name("add")
            .about("Adds package(s) to the system package list")
            .arg(Arg::with_name("PACKAGE")
                .help("Package(s) to add")
                .required(true)
                .min_values(1)
            )
        )
        .subcommand(SubCommand::with_name("check")
            .about("Displays information about a package")
            .arg(Arg::with_name("PACKAGE")
                .help("Package(s) to check")
                .required(true)
                .min_values(1)
            )
        )
        .subcommand(SubCommand::with_name("count")
            .about("Displays the number of installed packages")
        )
        .subcommand(SubCommand::with_name("list")
            .about("Displays the list of installed packages")
        )
        .subcommand(SubCommand::with_name("new")
            .about("Interactively adds a new package to the database")
        )
        .subcommand(SubCommand::with_name("delete")
            .about("Deletes a package from the database")
            .arg(Arg::with_name("PACKAGE")
                .help("Package to delete")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("search")
            .about("Searches for a package")
            .arg(Arg::with_name("SEARCH")
                .help("Search term")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("built-with")
            .about("Checks if one package is build with another")
            .arg(Arg::with_name("PACKAGE")
                .help("Package to check")
                .required(true)
            )
            .arg(Arg::with_name("DEPENDENCIES")
                .help("Dependencies to check against")
                .required(true)
                .min_values(1)
            )
        )
        .subcommand(SubCommand::with_name("tail")
            .about("Displays the last N lines of the log file")
            .arg(Arg::with_name("LINES")
                .help("Number of lines to display")
                .default_value("5")  // Will convert to int later
                .validator(is_pos_int)
            )
        )
        .subcommand(SubCommand::with_name("import")
            .about("Imports a package database from a csv file")
            .arg(Arg::with_name("PATH")
                .help("Path to CSV file")
                .required(true)
                .validator(file_exists)  // TODO: try adding a path validator here
            )
        );

    let matches = app.get_matches();

    let settings = Settings {
        verbose: matches.is_present("verbose"),
        no_confirm: matches.is_present("no_confirm"),
        runtime: matches.is_present("runtime"),
        show_deps: matches.is_present("show-deps"),
        color: matches.is_present("color")
    };

    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let packages: Vec<String> = sub_matches.values_of("PACKAGE").unwrap().map(|x| String::from(x)).collect();  // Get package arguments
            let packages = util::pkg::parse_packages(packages.as_slice());

            add(&settings, packages.as_slice());
        },
        ("built-with", Some(sub_matches)) => {
            let package = sub_matches.value_of("PACKAGE").unwrap();
            let package = util::pkg::parse_packages(vec![String::from(package)].as_slice());
            let package = package.first().unwrap().clone();

            let dependencies: Vec<String> = sub_matches.values_of("DEPENDENCIES").unwrap().map(|x| String::from(x)).collect();
            let dependencies = util::pkg::parse_packages(dependencies.as_slice());

            built_with(&settings, package, dependencies.as_slice());
        },
        ("check", Some(sub_matches)) => {
            let packages: Vec<String> = sub_matches.values_of("PACKAGE").unwrap().map(|x| String::from(x)).collect();
            let packages = util::pkg::parse_packages(packages.as_slice());

            check(&settings, packages.as_slice());
        },
        ("count", _) => {
            count(&settings);
        },
        ("list", _) => {
            list(&settings);
        },
        ("new", _) => {
            new(&settings);
        },
        ("delete", Some(sub_matches)) => {
            let packages: Vec<String> = sub_matches.values_of("PACKAGE").unwrap().map(|x| String::from(x)).collect();
            delete(&settings, packages.as_slice());
        },
        ("search", Some(sub_matches)) => {
            let packages: Vec<String> = sub_matches.values_of("PACKAGE").unwrap().map(|x| String::from(x)).collect();
            search(&settings, packages.as_slice());
        },
        ("tail", Some(sub_matches)) => {
            let lines = sub_matches.value_of("LINES").unwrap().parse::<u64>().unwrap();
            tail(&settings, lines);
        },
        ("import", Some(sub_matches)) => {
            let path = String::from(sub_matches.value_of("PATH").unwrap());

            import(&settings, &path);
        }
        _ => {}
    }
}

// ###### Subcommand Functions ######

fn add(settings: &Settings, packages: &[util::data::Package]) {
    // TODO: implement add
}

fn built_with(settings: &Settings, pkg: util::data::Package, deps: &[util::data::Package]) {
    // TODO: implement built with
}

fn check(settings: &Settings, packages: &[util::data::Package]) {
    // TODO: implement check
}

fn count(settings: &Settings) {
    // TODO: implement count
}

fn list(settings: &Settings) {
    // TODO: implement list
}

fn new(settings: &Settings) {
    // TODO: implement new
}

fn delete(settings: &Settings, package: &[String]) {
    // TODO: implement delete
}

fn search(settings: &Settings, package: &[String]) {
    // TODO: implement search
}

fn tail(settings: &Settings, lines: u64) {
    // TODO: implement tail
}

fn import(settings: &Settings, path: &String) {
    // TODO: implement import
    // TODO: 
    util::db::import_csv(path);
}

// Miscellaneous Functions

fn is_pos_int(s: String) -> Result<(), String> {
    let test = s.parse::<u64>().is_ok();

    return if test { return Ok(()); } else { Err(String::from("Value must be a positive integer")) };
}

fn file_exists(s: String) -> Result<(), String> {
    return if std::path::Path::new(s.as_str()).exists() { return Ok(()); } else { Err(String::from("Error: file not found!")) };
}