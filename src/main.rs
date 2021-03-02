use clap::{Arg, App, AppSettings, SubCommand};

mod util;

use util::data::Settings;

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
        .subcommand(SubCommand::with_name("tail")
            .about("Displays the last N lines of the log file")
            .arg(Arg::with_name("LINES")
                .help("Number of lines to display")
                .default_value("5")  // Will convert to int later
                .validator(util::is_pos_int)
            )
        )
        .subcommand(SubCommand::with_name("import")
            .about("Imports a package database from a csv file")
            .arg(Arg::with_name("PATH")
                .help("Path to CSV file")
                .required(true)
                .validator(util::file_exists)
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
        ("search", Some(sub_matches)) => {
            let package: String = String::from(sub_matches.value_of("SEARCH").unwrap());
            search(&settings, &package);
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

fn check(settings: &Settings, packages: &[util::data::Package]) {
    // TODO: implement check
    util::io::print_pkg_table(&packages, &settings);
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

fn search(settings: &Settings, package: &String) {
    let db = util::db::DB {
        path: util::paths::get_db_path(),
    };

    let results = db.find_key(package);
        for result in results {
        println!("{}", result);
    }    
}

fn import(settings: &Settings, path: &String) {
    print!("Merge or replace database? (m/r): ");
    use std::io::Write;
    std::io::stdout().flush().unwrap();

    let line: String = text_io::read!("{}\n");
    let line = line.to_ascii_lowercase();

    let mode: bool;
    match line.as_str() {
        "m" => {
            mode = false;
        }
        "r" => {
            mode = true;
        }
        _ => {
            eprintln!("Unrecognized input, exiting...");
            std::process::exit(1);
        }

    }


    let db = util::db::DB {
        path: util::paths::get_db_path(),
    };

    db.import_csv(path, mode);
}
