extern crate clap;
extern crate chrono;

use chrono::{NaiveDate};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use clap::{App, Arg, ArgMatches};

use api::{Api as AsanaApi};

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
            Arg::with_name("create")
            .long("create")
            .takes_value(false)
            .help("Create a task").requires("name")
        )
        .arg(
            Arg::with_name("due_date")
            .long("due_date")
            .takes_value(true)
            .help("Set a due date to a task. Must be YYYY-MM-DD")
        )
        .arg(
            Arg::with_name("update")
            .long("update")
            .takes_value(false)
            .help("Update a task").requires("task_id")
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
                .help("Specify a project")
        )
        .arg(
            Arg::with_name("note")
                .short("n")
                .long("note")
                .takes_value(true)
                .help("Add note on a task"),
        )
        .arg(
            Arg::with_name("name")
                .long("name")
                .takes_value(true)
                .help("Add/Change the task's name"),
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
    match api.tasks() {
        Ok(tasks) => println!("{}", tasks),
        Err(e) => println!("{}", e)
    }
}

fn display_workspaces(api: AsanaApi) -> () {
    match api.workspaces() {
        Ok(ws) => println!("{}", ws),
        Err(e) => println!("{}", e)
    }
}

fn display_projects(api: AsanaApi) -> () {
    match api.projects() {
        Ok(pts) => println!("{}", pts),
        Err(e) => println!("{}", e)
    }
}

fn display_users(api: AsanaApi) -> () {
    match api.users() {
        Ok(pts) => println!("{}", pts),
        Err(e) => println!("{}", e)
    }
}

fn main() {
    let conf = parsing_conf().expect("Cannot read config.conf");

    let matches = define_usage();

    let mut api = AsanaApi::new(conf["token"].to_owned(), conf["user_gid"].to_owned());

    if let Some(v) = matches.value_of("project_id") {
        api.project = Some(v.to_owned());
    }
    if matches.is_present("workspace_id") {
        api.set_workspace(matches.value_of("workspace_id").unwrap().to_owned());
    } else {
        api.set_workspace(conf["workspace"].to_owned());
    }
    let task_id = matches.value_of("task_id");
    if matches.is_present("tasks") {
        display_tasks(api);
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
    if matches.is_present("users") {
        display_users(api);
        return
    }
    let mut _jsn: HashMap<&str, serde_json::Value> = HashMap::new();
    if let Some(v) = matches.value_of("name") {
         _jsn.insert("name", serde_json::to_value(v).unwrap());
    }
    if let Some(v) = matches.value_of("note") {
        _jsn.insert("notes", serde_json::to_value(v).unwrap());
    }
    if let Some(v) = matches.value_of("due_date") {
        if NaiveDate::parse_from_str(v, "%Y-%m-%d").is_ok() {
             _jsn.insert("due_on", serde_json::to_value(v).unwrap());
        }
        else {
            println!("Cannot parse due_date parameter. Please use this format YYYY-MM-DD");
            return
        } 
    }
    if let Some(v) = matches.value_of("assignee") {
        _jsn.insert("assignee", serde_json::to_value(v).unwrap());
    }
    if let Some(c) = matches.value_of("finish") {
        let v: bool = c.parse().unwrap();
        _jsn.insert("completed", serde_json::to_value(v).unwrap());
    }
    if matches.is_present("update") {
        match api.update_task(task_id.unwrap().to_owned(), _jsn.clone()) {
            Err(e) => println!("{}", e),
            Ok(t) => println!("{}", t),
        };
        return
    }
    if matches.is_present("create") {
        match api.create_task(matches.value_of("name").unwrap().to_owned(), _jsn.clone()) {
            Err(e) => println!("{}", e),
            Ok(t) => println!("{}", t),
        };
    }
    if let Some(v) = matches.value_of("comment") {
        match api.add_comment(task_id.unwrap().to_owned(), v.to_owned()) {
            Err(e) => println!("{}", e),
            Ok(t) => println!("{}", t),
        };
    }
    if let Some(task_id) = matches.value_of("task_id"){
        if let Some(v) = api.project.clone() {
            match api.add_task_to_project(task_id.to_owned(), v.to_owned()) {
                Ok(task) => println!("{}", task),
                Err(e) => println!("{}", e)
            }
            return
        }
    }
    if matches.is_present("task_id") && !matches.is_present("create") && !matches.is_present("update") {
        match api.task(task_id.unwrap().to_owned()) {
            Ok(task) => println!("{}", task),
            Err(e) => println!("{}", e)
        }
        return
    }
}