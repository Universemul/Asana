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
                .help("Get all tasks"),
        )
        .arg(
            Arg::with_name("task_id")
                .short("t")
                .long("task")
                .takes_value(true)
                .help("Get Specific Task")
                .required_unless("tasks")
        )
        .arg(
            Arg::with_name("note")
                .short("m")
                .long("note")
                .takes_value(true)
                .help("Add note on specific task").requires("task_id"),
        )
        .arg(
            Arg::with_name("complete")
                .short("c")
                .takes_value(true)
                .help("Complete/Uncomplete a Task")
                .requires("task_id"),
        )
        .get_matches()
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
    let tasks: models::Tasks = api.get_tasks().unwrap();
    for tmp in tasks.data.into_iter() {
        let gid = tmp.gid;
        match api.get_task(&gid) {
            Err(e) => println!("error while get user task {}: {}", gid, e),
            Ok(task) => println!("{}", task),
        };
    }
}


fn main() {
    let conf = parsing_conf().unwrap();
    let api = AsanaApi {
        token: conf["token"].clone(),
        user_gid: conf["user_gid"].clone(),
    };
    let matches = define_usage();
    if matches.is_present("tasks") {
        display_tasks(api);
        return
    }
    let mut _jsn: HashMap<&str, serde_json::Value> = HashMap::new();
    let task_id = matches.value_of("task_id").unwrap();
    if let Some(v) = matches.value_of("note") {
        _jsn.insert("notes", serde_json::to_value(v).unwrap());
    }
    if let Some(c) = matches.value_of("complete") {
        let v: bool = c.parse().unwrap();
        _jsn.insert("completed", serde_json::to_value(v).unwrap());
    }
    let task = match api.update_task(&task_id, _jsn) {
        Err(e) => panic!("error while get user task {}: {}", task_id, e),
        Ok(t) => t,
    };
    println!("{:?}", task);
}
