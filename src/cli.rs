pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub arguments: Option<Vec<Argument <'a>>>,
    pub options: Option<Vec<Option_<'a>>>,
}

pub struct Argument<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

pub struct Option_<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

pub fn get_commands<'a>() -> Vec<Command<'a>> {
    let help = Command {
        name: "help",
        description: "Show the help page",
        options: None,
        arguments: None,
    };

    let usage = Command {
        name: "usage",
        description: "Show this page",
        options: None,
        arguments: None
    };

    let init = Command {
        name: "init",
        description: "Initialize passkeeper",
        options: None,
        arguments: None,
    };

    let add = Command {
        name: "add",
        description: "Add a password to the vault",
        options: None,
        arguments: None
    };

    let rm = Command {
        name: "rm, delete",
        description: "Remove a password from the vault",
        options: None, // TODO
        arguments: None, // TODO
    };

    let list = Command {
        name: "list, ls",
        description: "List saved passwords",
        options: None,
        arguments: None
    };

    vec!(usage, help, init, add, rm, list)
}

pub fn commands() {
    println!("\nCOMMANDS:");

    for c in get_commands() {
        println!("\t{}\t{}", &c.name, &c.description);
    }
}

pub fn usage() {
    println!(
        "USAGE:\n\tpasskeeper [global options]\n\tpasskeeper \
         [global options] command arguments... [command options]"
    );
}

pub fn help() {
    println!("NAME:\n\tpasskeeper - a secret manager written in Rust");
    println!("AUTHOR:\n\tTara Vancil <tbvanc@gmail.com>");
    usage();
    commands();
}
