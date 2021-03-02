pub fn print_pkg_table(packages: &[super::data::Package], settings: &super::data::Settings) {
    use prettytable::{Attr, Cell, Row, Table, color, format};

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(Row::new(vec![
        Cell::new("Name")
            .with_style(Attr::Bold),
        Cell::new("Installed")
            .with_style(Attr::Bold),
        Cell::new("Mode")
            .with_style(Attr::Bold)
    ]));

    for pkg in packages {
        if settings.color {
            table.add_row(Row::new(vec![
                Cell::new(pkg.name.as_str()),
                Cell::new(if super::pkg::is_installed(&pkg.name) { "Y" } else { "N" })
                    .with_style(Attr::ForegroundColor(
                        if super::pkg::is_installed(&pkg.name) {
                            color::GREEN
                        }
                        else {
                            color::RED
                        }
                    )
                ),
                Cell::new(if settings.verbose { "Explicit" } else { "E" })
            ]));
        }        
        else {
            table.add_row(Row::new(vec![
                Cell::new(pkg.name.as_str()),
                Cell::new(if super::pkg::is_installed(&pkg.name) { "Y" } else { "N" }),
                Cell::new(if settings.verbose { "Explicit" } else { "E" })
            ]));
        }
        
        if settings.show_deps {
            for dep in &pkg.dependencies {
                if settings.color {
                    table.add_row(Row::new(vec![
                        Cell::new(dep)
                            .with_style(Attr::ForegroundColor(color::BRIGHT_BLACK)),
                        Cell::new(if super::pkg::is_installed(dep) { "Y" } else { "N" })
                            .with_style(Attr::ForegroundColor(
                                if super::pkg::is_installed(dep) {
                                    color::GREEN
                                }
                                else {
                                    color::RED
                                }
                            )
                        ),
                        Cell::new(if settings.verbose { "Dependency" } else { "D" })
                            .with_style(Attr::ForegroundColor(color::BRIGHT_BLACK))
                    ]));
                }
                else {
                    table.add_row(Row::new(vec![
                        Cell::new(dep),
                        Cell::new(if super::pkg::is_installed(dep) { "Y" } else { "N" }),
                        Cell::new(if settings.verbose { "Dependency" } else { "D" })
                    ]));
                }
            }
        }
    }

    table.printstd();
}

pub fn get_lines(path: &str) -> Result<Vec<String>, String> {
    let contents = std::fs::read_to_string(path);

    match contents {
        Ok(c) => {
            let data: Vec<String> = c.lines().map(String::from).collect();
            return Ok(data)
        }
        Err(e) => {
            Err(String::from("Unable to read file"))
        }
    }

    

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_lines() {
        let lines: Vec<String> = super::get_lines("tests/files/test_get_lines.txt").unwrap();

        assert_eq!(3, lines.len());

        assert_eq!(String::from("line one"), lines[0]);
        assert_eq!(String::from("line two"), lines[1]);
        assert_eq!(String::from("line three"), lines[2]);
    }
}