use std::path;

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
        .subcommand(SubCommand::with_name("search")
            .about("Searches for a package")
            .arg(Arg::with_name("SEARCH")
                .help("Search term")
                .required(true)
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
    util::io::print_pkg_table(&packages, &settings);

    print!("Confirm changes? (Y/n): ");
    use std::io::Write;
    std::io::stdout().flush().unwrap();

    let line: String = text_io::read!("{}\n");
    let line = line.to_ascii_lowercase();

    if line.as_str() == "y" || line.as_str() == "" {
        let package_names: Vec<String> = packages.iter().map(|x| x.clone().name).collect();
        util::pkg::install(package_names.as_slice());
    }
    else if line.as_str() == "n" {
        eprintln!("Exiting...");
        std::process::exit(1);
    }
    else {
        eprintln!("Unrecognized input, exiting...");
        std::process::exit(1);
    }
}

fn check(settings: &Settings, packages: &[util::data::Package]) {
    util::io::print_pkg_table(&packages, &settings);
}

fn count(settings: &Settings) {
    let log = util::log::Log {
        path: util::paths::get_log_path(),
    };

    println!("{}{}", log.get_size(), if settings.verbose { " packages installed" } else { "" })
}

fn list(settings: &Settings) {
    let log = util::log::Log {
        path: util::paths::get_log_path(),
    };

    for line in log.get_installed() {
        println!("{}", line);
    }
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
