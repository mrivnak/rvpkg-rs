import argparse
import os
import re
import sys
from more_termcolor.colors import reverse
import yaml

from beautifultable import BeautifulTable
from more_termcolor import colored

from package import Package

# global configuration
verbose = False
no_confirm = False
default_yes = True
runtime = False
show_deps = False

debug = False
prefix = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'fs') if debug else '/'
config_path = os.path.join(prefix, 'etc', 'rvpkg.yaml')
db_path = os.path.join(prefix, 'usr', 'share', 'rvpkg', 'packages.yaml')
log_path = os.path.join(prefix, 'var', 'lib', 'rvpkg', 'packages.log')


# Add packages to the package list
def add_pkgs(pkgs):
    installed_pkgs = []
    for pkg in pkgs:
        if is_installed(pkg.name):
            installed_pkgs.append(pkg.name)

    if installed_pkgs:
        print('Package(s) "{}" already tracked, updating dependencies...'.format(
            ', '.join(installed_pkgs)
        ))
    print_pkgs(pkgs)

    confirm()

    with open(log_path, 'a+') as file:
        for pkg in pkgs:
            file.write(f'{pkg.name}\n')

    print('Packages successfully added!')


# Show information about multiple packages
def check_pkgs(pkgs):
    print_pkgs(pkgs)


# Prompt for confirmation
def confirm():
    if not no_confirm:
        print('Do you want to continue? {}: '.format('[Y/n]' if default_yes else '[y/N]'), end='')
        response = input()

        if not (response.lower() == "y" or (response == "" and default_yes)):
            print("Operation cancelled", file=sys.stderr)
            sys.exit(1)


# Displays number of installed packages
def count_pkgs():
    log = get_log()

    uniq_log = list(set(log))

    if verbose:
        print(f'{len(uniq_log)} packages currently installed')
    else:
        print(len(uniq_log))


# Returns the package log as a list of strings
def get_log():
    with open(log_path, 'r') as file:
        log = file.readlines()

    new_log = []
    for line in log:
        new_log.append(line.rstrip('\n'))

    log, new_log = new_log, None

    return log


# Checks if one package is built with another
def is_built_with(pkg, deps):
    if not is_installed(pkg.name):
        print(colored(f'Warning: Package "{pkg.name}" is not installed', 'yellow'))
        sys.exit(1)
    else:
        log = get_log()
        found = False
        index = None

        for i, n in enumerate(reversed(log)):
            if n == pkg.name:
                print(f'i: {i}, n: {n}')
                index = len(log) - 1 - i
                break

        log = log[:index]

        for dep in deps:
            if dep.entry in log:
                print(colored(f'\nPackage "{pkg}" is built with "{dep}"', 'green'))
            else:
                print(colored(f'\nPackage "{pkg}" is not built with "{dep}"', 'red'))

# Check if a specified package is installed
def is_installed(pkg):
    return pkg in get_log()


# Displays list of installed packages
def list_pkgs():
    pkgs = list(set(get_log()))
    pkgs.sort()
    for item in pkgs:
        print(item)


# Load config files
def load_config():
    global verbose, default_yes, no_confirm, runtime, show_deps

    with open(config_path, 'r') as file:
        data = yaml.load(file, Loader=yaml.FullLoader)

        verbose = data['config']['verbose']
        default_yes = data['config']['default_yes']
        no_confirm = data['config']['no_confirm']
        runtime = data['config']['runtime']
        show_deps = data['config']['show_deps']

# Add a new package to the database
def new_package():
    print('\nNew package')
    name = input('Name: ')
    print('NOTE: Input dependencies space delimited')
    req_deps = input('Required Dependencies: ').split()
    rec_deps = input('Recommended Dependencies: ').split()
    opt_deps = input('Optional Dependencies: ').split()
    req_run_deps = input('Required Runtime Dependencies: ').split()
    rec_run_deps = input('Recommended Runtime Dependencies: ').split()
    opt_run_deps = input('Optional Runtime Dependencies: ').split()

    print('\nVerify New Package')
    print(f'Name: {name}')
    if req_deps:
        print(f'Required Dependencies: {req_deps}')
    if rec_deps:
        print(f'Recommended Dependencies: {rec_deps}')
    if opt_deps:
        print(f'Optional Dependencies: {opt_deps}')
    if req_run_deps:
        print(f'Required Runtime Dependencies: {req_run_deps}')
    if rec_run_deps:
        print(f'Recommended Runtime Dependencies: {rec_run_deps}')
    if opt_run_deps:
        print(f'Optional Runtime Dependencies: {opt_run_deps}')

    # Could use pyyaml here, easier just with format string
    db_entry = f'\n  {name}:'
    if (
        not req_deps
        and not rec_deps
        and not opt_deps
        and not req_run_deps
        and not rec_run_deps
        and not opt_run_deps
    ):
        db_entry += ' {}'
    else:
        db_entry += ''
        if req_deps:
            db_entry += '\n    req:'
            for dep in req_deps:
                db_entry += f'\n      - {dep}'
        if rec_deps:
            db_entry += '\n    rec:'
            for dep in rec_deps:
                db_entry += f'\n      - {dep}'
        if opt_deps:
            db_entry += '\n    opt:'
            for dep in opt_deps:
                db_entry += f'\n      - {dep}'
        if req_run_deps:
            db_entry += '\n    req_run:'
            for dep in req_run_deps:
                db_entry += f'\n      - {dep}'
        if rec_run_deps:
            db_entry += '\n    rec_run:'
            for dep in rec_run_deps:
                db_entry += f'\n      - {dep}'
        if opt_run_deps:
            db_entry += '\n    opt_run:'
            for dep in opt_run_deps:
                db_entry += f'\n      - {dep}'

    confirm()

    with open(db_path, 'a') as file:
        file.write(db_entry)


# Setup argparse
def parse_args():
    # argparse setup
    global verbose, no_confirm, runtime, show_deps

    parser = argparse.ArgumentParser(prog='rvpkg')
    parser.add_argument(
        '-v',
        '--verbose',
        action="store_true",
        dest='verbose',
        default=False,
        help='Displays additional information'
    )
    parser.add_argument(
        '-n',
        '--no-confirm',
        action="store_true",
        dest='no_confirm',
        default=False,
        help='Accepts changes without prompting for confirmation'
    )
    '''
    parser.add_argument(
        '-c',
        '--color',
        action="store_true",
        dest='color',
        default=False,
        help='Prints output with color'
    )
    '''
    parser.add_argument(
        '-r',
        '--runtime',
        action="store_true",
        dest='runtime',
        default=False,
        help='Display runtime dependencies'
    )
    parser.add_argument(
        '-d',
        '--show-deps',
        action='store_true',
        dest='show_deps',
        default=False,
        help='Display package dependencies'
    )

    subparsers = parser.add_subparsers(help='rvpkg subcommands:')
    subparsers.required = True
    subparsers.dest = 'command'
    parser_add = subparsers.add_parser(
        'add',
        help='Adds package(s) to the system package list'
    )
    parser_check = subparsers.add_parser(
        'check',
        help='Displays information about package(s)'
    )
    parser_count = subparsers.add_parser(
        'count',
        help='Displays the number of installed packages'
    )
    parser_list = subparsers.add_parser(
        'list',
        help='Displays the list of installed packages'
    )
    parser_search = subparsers.add_parser(
        'search',
        help='Searches for a package'
    )
    parser_built_with = subparsers.add_parser(
        'built-with',
        help='Checks if one package is built with another'
    )
    parser_tail = subparsers.add_parser(
        'tail',
        help='Displays the last N numbers of the log file'
    )
    parser_new = subparsers.add_parser(
        'new',
        help='Adds a new package to the database'
    )

    # TODO: add new subcommand, interactively adds a package to the database

    parser_add.add_argument(
        'packages',
        type=str,
        action='append',
        nargs='+'
    )
    parser_check.add_argument(
        'packages',
        type=str,
        action='append',
        nargs='+'
    )
    parser_search.add_argument(
        'query',
        type=str
    )

    parser_built_with.add_argument(
        'package',
        type=str
    )
    parser_built_with.add_argument(
        'dependencies',
        type=str,
        action='append',
        nargs='+'
    )

    parser_tail.add_argument(
        'lines',
        type=int
    )

    parser_add.add_argument(
        '-d',
        '--show-deps',
        action='store_true',
        dest='add_show_deps',
        default=False,
        help='Display package dependencies'
    )
    parser_check.add_argument(
        '-d',
        '--show-deps',
        action='store_true',
        dest='check_show_deps',
        default=False,
        help='Display package dependencies'
    )

    args = parser.parse_args()

    verbose = verbose or args.verbose
    no_confirm = no_confirm or args.no_confirm
    runtime = runtime or args.runtime
    show_deps = show_deps or args.show_deps

    command = args.command

    if command in ['add', 'check']:
        data = args.packages[0]
        if command == 'add':
            show_deps = show_deps or args.add_show_deps
        elif command == 'check':
            show_deps = show_deps or args.check_show_deps
    elif command == 'search':
        data = [args.query]
    elif command == 'built-with':
        data = [args.package] + args.dependencies[0]
    elif command == 'tail':
        data = [args.lines]
    else:
        data = None

    return command, data


# Convert package strings to package objects
def parse_pkgs(pkgs):
    output = []
    data = None
    package = None

    with open(db_path, 'r') as file:
        data = yaml.load(file, Loader=yaml.FullLoader)

    package_names = data['packages'].keys()

    for pkg in pkgs:
        matches = [x for x in package_names if pkg in x]

        if len(matches) == 0:
            print(f'Package "{pkg}" not found in database. Exiting...', file=sys.stderr)
            sys.exit(1)
        elif len(matches) == 1:
            name = matches[0]
            package = Package(name)
        else:
            print(f'\nPackage "{pkg}" has multiple matches')

            for i, pkg in enumerate(matches):
                print(f'{i + 1}) {pkg}')

            try:
                index = int(input('Package # to select: '))
            except TypeError:
                print('Error: Invalid selection', file=sys.stderr)
                sys.exit(1)

            if 1 <= index <= len(matches):
                name = matches[index - 1]
                package = Package(name)
            else:
                print('Error: Invalid selection', file=sys.stderr)
                sys.exit(1)

        package.installed = is_installed(package.entry)

        package.req_deps = data['packages'][package.entry].get('req', [])
        package.rec_deps = data['packages'][package.entry].get('rec', [])
        package.opt_deps = data['packages'][package.entry].get('opt', [])

        output.append(package)
        package = None

    return output


# Display a package and details to the screen
def print_pkgs(pkgs):
    table = BeautifulTable()
    table.columns.header = ['Name', 'Type', 'Installed']

    for pkg in pkgs:
        table.rows.append([pkg.name, 'E', colored('Yes', 'green') if pkg.installed else colored('No', 'red')])
        if show_deps and len(pkg.req_deps + pkg.rec_deps + pkg.opt_deps) > 0:
            table.rows.append([colored(f'{pkg.name} dependencies', 'bright black'), '', ''])
            for item in (pkg.req_deps + pkg.rec_deps + pkg.opt_deps):
                name = item
                table.rows.append([colored(name, 'bright black'), colored('D', 'bright black'), colored('Yes', 'green') if is_installed(item) else colored('No', 'red')])

    # Table styling
    table.columns.alignment = BeautifulTable.ALIGN_LEFT
    table.set_style(BeautifulTable.STYLE_NONE)
    table.columns.header.separator = '='
    table.columns.padding = 2

    print('')
    print(table)


# Looks for packages with the query in the name
def search(query):
    with open(db_path, 'r') as file:
        data = yaml.load(file, Loader=yaml.FullLoader)

    for item in list(data['packages']):
        if query in item:
            print(item)


# Displays the last N lines of the package log
def tail(lines):
    for pkg in get_log()[-lines:]:
        print(pkg)


def main():
    load_config()
    cmd, data = parse_args()
    pkgs = None

    if data and cmd != 'search' and cmd != 'tail' and cmd != 'new':
        pkgs = parse_pkgs(data)

    if cmd == 'add':
        add_pkgs(pkgs)
    elif cmd == 'check':
        check_pkgs(pkgs)
    elif cmd == 'count':
        count_pkgs()
    elif cmd == 'list':
        list_pkgs()
    elif cmd == 'search':
        search(data[0])
    elif cmd == 'built-with':
        is_built_with(pkgs[0], pkgs[1:])
    elif cmd == 'tail':
        tail(data[0])
    elif cmd == 'new':
        new_package()


if __name__ == '__main__':
    main()
