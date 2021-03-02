pub fn print_pkg_table(packages: &[super::data::Package], settings: &super::data::Settings) {
    use prettytable::{Attr, Cell, Row, Table, format};

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
        let color = get_color(settings.color, super::pkg::is_installed(&pkg.name));

        table.add_row(Row::new(vec![
            Cell::new(pkg.name.as_str()),
            Cell::new(pkg.name.as_str())
                .with_style(Attr::ForegroundColor(color))
        ]));
        
        if settings.show_deps {

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

fn get_color(use_color: bool, is_installed: bool) -> u32 {
    use prettytable::color;

    if use_color && is_installed {
        return color::GREEN;
    }
    else if use_color && !is_installed {
        return color::RED;
    }
    else {
        return color::WHITE;
    };
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