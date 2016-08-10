use std::collections::HashMap;
use std::io::{ Error, ErrorKind };

pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub required: Vec<&'a str>,
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

impl<'a> Command<'a> {
    pub fn usage(&self) {
        println!("USAGE:");
        println!("passkeeper {} {:?}", &self.name, &self.required);
    }
}

pub fn get_commands<'a>() -> HashMap<&'a str, Command<'a>> {
    let help = Command {
        name: "help",
        description: "Show the help page",
        required: vec!(),
        options: None,
        arguments: None,
    };

    let usage = Command {
        name: "usage",
        description: "Show this page",
        required: vec!(),
        options: None,
        arguments: None
    };

    let init = Command {
        name: "init",
        description: "Initialize passkeeper",
        required: vec!(),
        options: None,
        arguments: None,
    };

    let add = Command {
        name: "add",
        description: "Add a password to the vault",
        required: vec!("<site>"),
        options: None,
        arguments: None
    };

    let rm = Command {
        name: "rm",
        description: "Remove a password from the vault",
        required: vec!("<site>"),
        options: None, // TODO
        arguments: None, // TODO
    };

    let show = Command {
        name: "show",
        description: "Show the password for a given site",
        required: vec!("<site>"),
        options: None, // TODO
        arguments: None,
    };

    let ls = Command {
        name: "ls",
        description: "List saved passwords",
        required: vec!(),
        options: None,
        arguments: None
    };

    let mut commands_list = HashMap::new();
    let commands = vec!(add, rm, show, ls, init, help, usage);

    for c in commands {
        commands_list.insert(c.name, c);
    }
    commands_list
}

pub fn commands() {
    println!("\nCOMMANDS:");

    for (name, c) in &get_commands() {
        println!("\t{}\t{}", name, &c.description);
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

pub fn check_args(command: &Command, args: &[String]) -> Result<(), Error> {
    if args.len() < command.required.len() {
        println!("Error: Missing required argument(s)");
        command.usage();
        return Err(Error::new(ErrorKind::InvalidInput, "Missing required argument"))
    }
    Ok(())
}
