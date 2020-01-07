extern crate reqwest;

pub mod errors;
pub mod models;

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;


pub struct Api{
    pub token: String,
    pub user_gid: String,
    pub workspace: Option<String>,
    pub project: Option<String>
}

impl Api {
    const BASE_URL: &'static str = "https://app.asana.com/api/1.0";

    pub fn new(token: String, user_gid: String) -> Api {
        Api {
            token: token,
            user_gid: user_gid,
            workspace: None,
            project: None
        }
    }

    pub fn set_workspace(&mut self, gid: String) -> &mut Self {
        self.workspace = Some(gid);
        self
    }

    fn _build_request(&self, task_id: String, resource: Option<&str>) -> String {
        let url = match resource {
            Some(v) => format!("{}/tasks/{}/{}", Api::BASE_URL, task_id, v).to_owned(),
            None => format!("{}/tasks/{}", Api::BASE_URL, task_id).to_owned()
        };
        url.to_owned()
    }

    fn get<T>(&self, url: &str) -> Result<T, errors::ApiError> 
        where for<'de> T: serde::Deserialize<'de>{
        let mut response = Client::new().get(url).bearer_auth(&self.token).send().unwrap();
        if !response.status().is_success() {
            return Err(errors::ApiError {
                code: response.status().as_u16(),
                message: response.text().unwrap()
            })
        };
        let result:T = response.json()?;
        Ok(result)
    }

    fn list<T>(&self, url: &str) -> Result<T, errors::ApiError> 
        where for<'de> T: serde::Deserialize<'de> {
        let response: T = self.get(url)?;
        Ok(response)
    }

    fn put<T>(&self, url: &str, params: HashMap<&str, Value>) -> Result<T, errors::ApiError>
        where for<'de> T: serde::Deserialize<'de> {
        let mut body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        body.insert("data", params);
        let mut response = Client::new().put(url).json(&body).bearer_auth(&self.token).send().unwrap();
        if !response.status().is_success() {
            return Err(errors::ApiError {
                code: response.status().as_u16(),
                message: response.text().unwrap()
            })
        };
        let result:T = response.json()?;
        Ok(result)
    }

    fn post<T>(&self, url: &str, params: HashMap<&str, Value>) -> Result<T, errors::ApiError>
        where for<'de> T: serde::Deserialize<'de> {
        let mut body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        body.insert("data", params);
        let mut response = Client::new().post(url).json(&body).bearer_auth(&self.token).send().unwrap();
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
        if let Some(v) = &self.project {
            url = format!("{}/projects/{}/tasks", Api::BASE_URL, v);
        }
        self.list::<models::Tasks>(&url)
    }

    pub fn task(&self, task_id: String) -> Result<models::Task, errors::ApiError> {
        let url = &format!("{}/tasks/{}", Api::BASE_URL, task_id);
        let body: HashMap<String, Value> = self.get(&url)?;
        let result: models::Task = serde_json::from_value(body["data"].to_owned()).unwrap();
        Ok(result)
    }

    pub fn workspaces(&self) -> Result<models::Workspaces, errors::ApiError>{
        let url = &format!("{}/workspaces", Api::BASE_URL);
        self.list::<models::Workspaces>(&url)
    } 

    pub fn projects(&self) -> Result<models::Projects, errors::ApiError> {
        let url = &format!("{}/projects?workspace={}", Api::BASE_URL, self.workspace.as_ref().unwrap());
        self.list::<models::Projects>(&url)
    } 

    pub fn users(&self) -> Result<models::Users, errors::ApiError>{
        let url = &format!("{}/users?workspace={}", Api::BASE_URL, self.workspace.as_ref().unwrap());
        self.list::<models::Users>(&url)
    } 

    pub fn update_task(&self, task_id: String, body: HashMap<&str, Value>) -> Result<models::Task, errors::ApiError> {
        let url = self._build_request(task_id, None);
        let response: HashMap<String, Value> = self.put(&url, body)?;
        let result: models::Task = serde_json::from_value(response["data"].clone()).unwrap();
        return Ok(result)
    }

    pub fn add_comment(&self, task_id: String, comment: String) -> Result<bool, errors::ApiError> {
        let url = self._build_request(task_id, Some("stories"));
        let mut body: HashMap<&str, Value> = HashMap::new();
        body.insert("text", serde_json::to_value(comment).unwrap());
        let response: HashMap<String, Value> = self.post(&url, body)?;
        return Ok(match response.get("data") {
            Some(_) => true,
            None => false
        })
    }

    pub fn add_task_to_project(&self, task_id: String, project_id: String) -> Result<bool, errors::ApiError> {
        let url = self._build_request(task_id, Some("addProject"));
        let mut body: HashMap<&str, Value> = HashMap::new();
        body.insert("project", serde_json::to_value(project_id).unwrap());
        let response: HashMap<String, Value> = self.post(&url, body)?;
        return Ok(match response.get("data") {
            Some(_) => true,
            None => false
        })
    }

    pub fn create_task(&self, name: String, data: HashMap<&str, Value>) -> Result<models::Task, errors::ApiError>{
        let url = format!("{}/tasks", Api::BASE_URL);
        let mut body: HashMap<&str, Value> = HashMap::new();
        body.extend(data.into_iter().map(|(k, v)| (k.clone(), v.clone())));
        body.insert("name", serde_json::to_value(name).unwrap());
        if let Some(x) = &self.project {
            body.insert("projects", serde_json::json!([x]));
        };
        body.insert("workspace", serde_json::to_value(self.workspace.as_ref()).unwrap());
        
        let response: HashMap<String, Value> = self.post(&url, body)?;
        let data = match response.get("data") {
            Some(tmp) => tmp,
            None => panic!("[CREATE TASK] Cannot create task")
        };
        let result: models::Task = serde_json::from_value(data.clone()).unwrap();
        return Ok(result)
    }
}
