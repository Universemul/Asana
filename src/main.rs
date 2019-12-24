extern crate clap;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use clap::{App, Arg, ArgMatches};

mod api;
mod models;

use api::Api as AsanaApi;

fn define_usage() -> ArgMatches<'static> {
    App::new("Asana Awesome Toll")
        .version("1.0.0")
        .author("David Barthelemy <davidbarthelemy28@gmail.com>")
        .arg(
            Arg::with_name("tasks")
                .long("tasks")
                .takes_value(false)
                .help("Display all tasks"),
        )
        .arg(
            Arg::with_name("workspaces")
                .long("workspaces")
                .takes_value(false)
                .help("Display all workspace"),
        )
        .arg(
            Arg::with_name("projects")
                .long("projects")
                .takes_value(false)
                .help("Display all Projects"),
        )
        .arg(
            Arg::with_name("users")
                .long("users")
                .takes_value(false)
                .help("Display all users"),
        )
        .arg(
            Arg::with_name("task_id")
                .short("t")
                .takes_value(true)
                .help("Specify a task"),
        )
        .arg(
            Arg::with_name("workspace_id")
                .short("w")
                .takes_value(true)
                .help("Specify a workspace"),
        )
        .arg(
            Arg::with_name("assignee")
                .short("a")
                .takes_value(true)
                .help("Assignee to a user"),
        )
        .arg(
            Arg::with_name("project_id")
                .short("p")
                .takes_value(true)
                .help("Specify a project"),
        )
        .arg(
            Arg::with_name("note")
                .short("n")
                .long("note")
                .takes_value(true)
                .help("Add note on a task").requires("task_id"),
        )
        .arg(
            Arg::with_name("comment")
                .short("c")
                .long("comment")
                .takes_value(true)
                .help("Add comment on a task").requires("task_id"),
        )
        .arg(
            Arg::with_name("finish")
                .long("finish")
                .short("f")
                .takes_value(true)
                .help("Complete/Uncomplete a Task. Accepts true or false")
                .requires("task_id"),
        ).get_matches()
}

fn parsing_conf() -> Result<HashMap<String, String>, Error> {
    let path = "config.conf";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut result = HashMap::new();

    let lines = buffered.lines();
    for line in lines {
        let l = line?;
        let split: Vec<&str> = l.split("=").collect();
        let key = split[0].trim().to_lowercase();
        let value = split[1].trim().to_lowercase();
        result.insert(key.to_owned(), value.to_owned());
    }
    Ok(result)
}

fn display_tasks(api: AsanaApi) -> () {
    let tasks: models::Tasks = api.tasks().unwrap();
    println!("{}", tasks);
}

fn display_projects(api: AsanaApi) -> () {
    let projects: models::Projects = api.projects().unwrap();
    println!("{}", projects);
}

fn display_workspaces(api: AsanaApi) -> () {
    let workspaces: models::Workspaces = api.workspaces().unwrap();
    println!("{}", workspaces)
}


fn main() {
    let conf = parsing_conf().unwrap();
    let matches = define_usage();
    let mut api = AsanaApi::new(&conf["token"], &conf["user_gid"], &conf["workspace"]);
    let users: models::Users = api.users().unwrap();
    if matches.args.is_empty() {
        println!("No args is passed. Please see --help");
        return
    }
    if let Some(v) = matches.value_of("workspace") {
        api.workspace = v
    }
    api.project = matches.value_of("project_id");
    if matches.is_present("tasks") {
        display_tasks(api);
        return
    }
    if matches.is_present("users") {
        println!("{}", users);
        return
    }
    if matches.is_present("workspaces") {
        display_workspaces(api);
        return
    }
    if matches.is_present("projects") {
        display_projects(api);
        return
    }
    let mut _jsn: HashMap<&str, serde_json::Value> = HashMap::new();
    let task_id = matches.value_of("task_id");
    if let Some(v) = matches.value_of("comment") {
        let result = api.add_comment(task_id, v).unwrap();
        if !result {
            println!("Fail to add comment to the task {}", task_id.unwrap());
        }
    }
    if let Some(v) = matches.value_of("note") {
        _jsn.insert("notes", serde_json::to_value(v).unwrap());
    }
    if let Some(v) = matches.value_of("assignee") {
        let mut real_value = v;
        if let Some(resource) = users.data.iter().find(|x| x.name == v) {
            real_value = &resource.gid
        };
        _jsn.insert("assignee", serde_json::to_value(real_value).unwrap());
    }
    if let Some(c) = matches.value_of("finish") {
        let v: bool = c.parse().unwrap();
        _jsn.insert("completed", serde_json::to_value(v).unwrap());
    }
    if !_jsn.is_empty() {
        match api.update_task(task_id, _jsn) {
            Err(e) => panic!("error while update Task {}", e),
            Ok(t) => t,
        };
    }
    let t = api.get_task(task_id).unwrap();
    println!("{}", t)
}

/*
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use console::{style, Term};

const RESOURCES: [&str; 4] = ["Tasks", "Projects", "Workspaces", "Users"];

fn welcome(term : &Term) -> io::Result<()> {
    let term = Term::stdout();
    term.write_line("Hello ! Welcome in Asana Tool. What ressource do you want to query ?")?;
    for x in RESOURCES.iter() {
        term.write_line(&format!("{}", style(x).cyan()))?;
    }
    Ok(())
}

fn main() {
    let mut term = Term::stdout();
    welcome(&term).unwrap()
}*/