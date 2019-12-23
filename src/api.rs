extern crate reqwest;

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

use crate::models;

pub struct Api<'a>{
    pub token: &'a str,
    pub user_gid: &'a str,
    pub workspace: &'a str,
    pub project: Option<&'a str>
}


impl<'a> Api<'a> {
    const BASE_URL: &'static str = "https://app.asana.com/api/1.0";

    pub fn new(token: &'a str, user_gid: &'a str, workspace: &'a str) -> Api<'a> {
        Api {
            token: token,
            user_gid: user_gid,
            workspace: workspace,
            project: None
        }
    }

    fn get<T>(&self, url: &str) -> Result<T, reqwest::Error> where 
        for<'de> T: serde::Deserialize<'de> {
        let response = Client::new().get(url).bearer_auth(&self.token).send()?.json()?;
        return Ok(response)
    }

    fn put<T>(&self, url: &str, body: HashMap<&str, HashMap<&str, Value>>) -> Result<T, reqwest::Error>
        where for<'de> T: serde::Deserialize<'de> {
        let response = Client::new().put(url).json(&body).bearer_auth(&self.token).send()?.json()?;
        return Ok(response)
    }

    fn post<T>(&self, url: &str, body: HashMap<&str, HashMap<&str, Value>>) -> Result<T, reqwest::Error>
        where for<'de> T: serde::Deserialize<'de> {
        let response = Client::new().post(url).json(&body).bearer_auth(&self.token).send()?.json()?;
        return Ok(response)
    }

    pub fn tasks(&self) -> Result<models::Tasks, reqwest::Error>{
        let mut url = format!("{}/user_task_lists/{}/tasks?completed_since=now", Api::BASE_URL, self.user_gid);
        if let Some(v) = self.project {
            url = format!("{}/projects/{}/tasks", Api::BASE_URL, v);
        }
        self.get(&url)
    }

    pub fn projects(&self) -> Result<models::Projects, reqwest::Error>{
        let url = &format!("{}/projects?workspace={}", Api::BASE_URL, self.workspace);
        self.get(url)
    } 

    pub fn users(&self) -> Result<models::Users, reqwest::Error>{
        let url = &format!("{}/users?workspace={}", Api::BASE_URL, self.workspace);
        self.get(url)
    } 

    pub fn workspaces(&self) -> Result<models::Workspaces, reqwest::Error>{
        let url = &format!("{}/workspaces", Api::BASE_URL);
        self.get(url)
    }   

    pub fn get_task(&self, task_id: Option<&str>) -> Result<models::Task, reqwest::Error> {
        task_id.expect("Task id is required");
        let url = &format!("{}/tasks/{}", Api::BASE_URL, task_id.unwrap());
        let response: HashMap<String, Value> = self.get(url)?;
        let data = match response.get("data") {
            Some(tmp) => tmp,
            None => panic!("[GET USER TASK] Task with id {} is not recognized", task_id.unwrap())
        };
        let result: models::Task = serde_json::from_value(data.to_owned()).unwrap();
        return Ok(result)
    }

    pub fn update_task(&self, task_id: Option<&str>, body: HashMap<&str, Value>) -> Result<models::Task, reqwest::Error> {
        task_id.expect("Task id is required to add a comment");
        let url = &format!("{}/tasks/{}", Api::BASE_URL, task_id.unwrap());
        let mut new_body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        new_body.insert("data", body);
        let response: HashMap<String, Value> = self.put(url, new_body)?;
        let data = match response.get("data") {
            Some(tmp) => tmp,
            None => panic!("[UPDATE TASK] Task with id {} is not recognized", task_id.unwrap())
        };
        let result: models::Task = serde_json::from_value(data.clone()).unwrap();
        return Ok(result)
    }

    pub fn add_comment(&self, task_id: Option<&str>, comment: &str) -> Result<bool, reqwest::Error> {
        task_id.expect("Task id is required to add a comment");
        let url = &format!("{}/tasks/{}/stories", Api::BASE_URL, task_id.unwrap());
        let mut data_body: HashMap<&str, Value> = HashMap::new();
        data_body.insert("text", serde_json::to_value(comment).unwrap());
        let mut body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        body.insert("data", data_body);
        let response: HashMap<String, Value> = self.post(url, body)?;
        return Ok(match response.get("data") {
            Some(_) => true,
            None => false
        })
    }
}
