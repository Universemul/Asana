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

    fn get<T>(&self, url: String) -> Result<T, reqwest::Error> where 
        for<'de> T: serde::Deserialize<'de> {
        let response = Client::new().get(url.as_str()).bearer_auth(&self.token).send()?.json()?;
        return Ok(response)
    }

    fn put<T>(&self, url: String, params: HashMap<&str, Value>) -> Result<T, reqwest::Error>
        where for<'de> T: serde::Deserialize<'de> {
        let mut body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        body.insert("data", params);
        let response = Client::new().put(url.as_str()).json(&body).bearer_auth(&self.token).send()?.json()?;
        return Ok(response)
    }

    fn post<T>(&self, url: String, params: HashMap<&str, Value>) -> Result<T, reqwest::Error>
        where for<'de> T: serde::Deserialize<'de> {
        let mut body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        body.insert("data", params);
        let response = Client::new().post(url.as_str()).json(&body).bearer_auth(&self.token).send()?.json()?;
        return Ok(response)
    }

    pub fn get_task(&self, task_id: Option<&str>) -> Result<models::Task, reqwest::Error> {
        task_id.expect("Task id is required");
        let url = &format!("{}/tasks/{}", Api::BASE_URL, task_id.unwrap());
        let response: HashMap<String, Value> = self.get(url.to_string())?;
        let data = match response.get("data") {
            Some(tmp) => tmp,
            None => panic!("[GET USER TASK] Task with id {} is not recognized", task_id.unwrap())
        };
        let result: models::Task = serde_json::from_value(data.to_owned()).unwrap();
        return Ok(result)
    }

    fn _build_request(&self, task_id: Option<&str>, resource: Option<&str>) -> String {
        task_id.expect("Task id is required to add a comment");
        let url = match resource {
            Some(v) => format!("{}/tasks/{}/{}", Api::BASE_URL, task_id.unwrap(), v).to_owned(),
            None => format!("{}/tasks/{}", Api::BASE_URL, task_id.unwrap()).to_owned()
        };
        url.to_string()
    }

    pub fn update_task(&self, task_id: Option<&str>, body: HashMap<&str, Value>) -> Result<models::Task, reqwest::Error> {
        let url = self._build_request(task_id, None);
        let response: HashMap<String, Value> = self.put(url, body)?;
        let data = match response.get("data") {
            Some(tmp) => tmp,
            None => panic!("[UPDATE TASK] Task with id {} is not recognized", task_id.unwrap())
        };
        let result: models::Task = serde_json::from_value(data.clone()).unwrap();
        return Ok(result)
    }

    pub fn add_comment(&self, task_id: Option<&str>, comment: &str) -> Result<bool, reqwest::Error> {
        let url = self._build_request(task_id, None);
        let mut body: HashMap<&str, Value> = HashMap::new();
        body.insert("text", serde_json::to_value(comment).unwrap());
        let response: HashMap<String, Value> = self.post(url, body)?;
        return Ok(match response.get("data") {
            Some(_) => true,
            None => false
        })
    }

    pub fn add_task_to_project(&self, task_id: Option<&str>, project_id: &str) -> Result<bool, reqwest::Error> {
        let url = self._build_request(task_id, Some("addProject"));
        let mut body: HashMap<&str, Value> = HashMap::new();
        body.insert("project", serde_json::to_value(project_id).unwrap());
        let response: HashMap<String, Value> = self.post(url, body)?;
        return Ok(match response.get("data") {
            Some(_) => true,
            None => false
        })
    }

        
    pub fn tasks(&self) -> Result<models::Tasks, reqwest::Error>{
        let mut url = format!("{}/user_task_lists/{}/tasks?completed_since=now", Api::BASE_URL, self.user_gid);
        if let Some(v) = self.project {
            url = format!("{}/projects/{}/tasks", Api::BASE_URL, v);
        }
        self.get(url.to_string())
    }

    pub fn projects(&self) -> Result<models::Projects, reqwest::Error>{
        let url = &format!("{}/projects?workspace={}", Api::BASE_URL, self.workspace);
        self.get(url.to_string())
    } 

    pub fn users(&self) -> Result<models::Users, reqwest::Error>{
        let url = &format!("{}/users?workspace={}", Api::BASE_URL, self.workspace);
        self.get(url.to_string())
    } 

    pub fn workspaces(&self) -> Result<models::Workspaces, reqwest::Error>{
        let url = &format!("{}/workspaces", Api::BASE_URL);
        self.get(url.to_string())
    }   
}
