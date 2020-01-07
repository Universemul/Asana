extern crate reqwest;

pub mod errors;
pub mod models;

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;


pub struct Api<'a>{
    pub token: &'a str,
    pub user_gid: &'a str,
    pub workspace: Option<&'a str>,
    pub project: Option<&'a str>
}

impl<'a> Api<'a> {
    const BASE_URL: &'static str = "https://app.asana.com/api/1.0";

    pub fn new(token: &'a str, user_gid: &'a str) -> Api<'a> {
        Api {
            token: token,
            user_gid: user_gid,
            workspace: None,
            project: None
        }
    }

    pub fn set_workspace(&mut self, gid: &'a str) -> &mut Self {
        self.workspace = Some(gid);
        self
    }

    fn _build_request(&self, task_id: &str, resource: Option<&str>) -> String {
        let url = match resource {
            Some(v) => format!("{}/tasks/{}/{}", Api::BASE_URL, task_id, v).to_owned(),
            None => format!("{}/tasks/{}", Api::BASE_URL, task_id).to_owned()
        };
        url.to_string()
    }

    fn get<T>(&self, url: String) -> Result<T, errors::ApiError> 
        where for<'de> T: serde::Deserialize<'de>{
        let mut response = Client::new().get(url.as_str()).bearer_auth(&self.token).send().unwrap();
        if !response.status().is_success() {
            return Err(errors::ApiError {
                code: response.status().as_u16(),
                message: response.text().unwrap()
            })
        };
        let result:T = response.json()?;
        Ok(result)
    }

    fn list<T>(&self, url: String) -> Result<T, errors::ApiError> 
        where for<'de> T: serde::Deserialize<'de> {
        let response: T = self.get(url)?;
        Ok(response)
    }

    fn put<T>(&self, url: String, params: HashMap<&str, Value>) -> Result<T, errors::ApiError>
        where for<'de> T: serde::Deserialize<'de> {
        let mut body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        body.insert("data", params);
        let mut response = Client::new().put(url.as_str()).json(&body).bearer_auth(&self.token).send().unwrap();
        if !response.status().is_success() {
            return Err(errors::ApiError {
                code: response.status().as_u16(),
                message: response.text().unwrap()
            })
        };
        let result:T = response.json()?;
        Ok(result)
    }

    fn post<T>(&self, url: String, params: HashMap<&str, Value>) -> Result<T, errors::ApiError>
        where for<'de> T: serde::Deserialize<'de> {
        let mut body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        body.insert("data", params);
        let mut response = Client::new().post(url.as_str()).json(&body).bearer_auth(&self.token).send().unwrap();
        if !response.status().is_success() {
            return Err(errors::ApiError {
                code: response.status().as_u16(),
                message: response.text().unwrap()
            })
        };
        let result:T = response.json()?;
        Ok(result)
    }

    pub fn tasks(&self) -> Result<models::Tasks, errors::ApiError>{
        let mut url = format!("{}/user_task_lists/{}/tasks?completed_since=now", Api::BASE_URL, self.user_gid);
        if let Some(v) = self.project {
            url = format!("{}/projects/{}/tasks", Api::BASE_URL, v);
        }
        self.list::<models::Tasks>(url.to_string())
    }

    pub fn task(&self, task_id: &str) -> Result<models::Task, errors::ApiError> {
        let url = &format!("{}/tasks/{}", Api::BASE_URL, task_id);
        let body: HashMap<String, Value> = self.get(url.to_string())?;
        let result: models::Task = serde_json::from_value(body["data"].to_owned()).unwrap();
        Ok(result)
    }

    pub fn workspaces(&self) -> Result<models::Workspaces, errors::ApiError>{
        let url = &format!("{}/workspaces", Api::BASE_URL);
        self.list::<models::Workspaces>(url.to_string())
    } 

    pub fn projects(&self) -> Result<models::Projects, errors::ApiError> {
        let url = &format!("{}/projects?workspace={}", Api::BASE_URL, self.workspace.unwrap());
        self.list::<models::Projects>(url.to_string())
    } 

    pub fn users(&self) -> Result<models::Users, errors::ApiError>{
        let url = &format!("{}/users?workspace={}", Api::BASE_URL, self.workspace.unwrap());
        self.list::<models::Users>(url.to_string())
    } 

    pub fn update_task(&self, task_id: &str, body: HashMap<&str, Value>) -> Result<models::Task, errors::ApiError> {
        let url = self._build_request(task_id, None);
        let response: HashMap<String, Value> = self.put(url.to_string(), body)?;
        let result: models::Task = serde_json::from_value(response["data"].clone()).unwrap();
        return Ok(result)
    }

    pub fn add_comment(&self, task_id: &str, comment: &str) -> Result<bool, errors::ApiError> {
        let url = self._build_request(task_id, Some("stories"));
        let mut body: HashMap<&str, Value> = HashMap::new();
        body.insert("text", serde_json::to_value(comment).unwrap());
        let response: HashMap<String, Value> = self.post(url.to_string(), body)?;
        return Ok(match response.get("data") {
            Some(_) => true,
            None => false
        })
    }

    pub fn add_task_to_project(&self, task_id: &str, project_id: &str) -> Result<bool, errors::ApiError> {
        let url = self._build_request(task_id, Some("addProject"));
        let mut body: HashMap<&str, Value> = HashMap::new();
        body.insert("project", serde_json::to_value(project_id).unwrap());
        let response: HashMap<String, Value> = self.post(url.to_string(), body)?;
        return Ok(match response.get("data") {
            Some(_) => true,
            None => false
        })
    }

    pub fn create_task(&self, name: &str, data: HashMap<&str, Value>) -> Result<models::Task, errors::ApiError>{
        let url = format!("{}/tasks", Api::BASE_URL);
        let mut body: HashMap<&str, Value> = HashMap::new();
        body.extend(data.into_iter().map(|(k, v)| (k.clone(), v.clone())));
        body.insert("name", serde_json::to_value(name).unwrap());
        if let Some(x) = self.project {
            body.insert("projects", serde_json::json!([x]));
        };
        body.insert("workspace", serde_json::to_value(self.workspace).unwrap());
        
        let response: HashMap<String, Value> = self.post(url, body)?;
        let data = match response.get("data") {
            Some(tmp) => tmp,
            None => panic!("[CREATE TASK] Cannot create task")
        };
        let result: models::Task = serde_json::from_value(data.clone()).unwrap();
        return Ok(result)
    }
}
