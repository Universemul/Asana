extern crate reqwest;

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

use crate::models;

pub struct Api{
    pub token: String,
    pub user_gid: String
}

impl Api {
    const BASE_URL: &'static str = "https://app.asana.com/api/1.0";

    fn get<T>(&self, url: &str) -> Result<T, reqwest::Error> where 
        for<'de> T: serde::Deserialize<'de>, 
        T: std::fmt::Debug{
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

    pub fn get_tasks(&self) -> Result<models::Tasks, reqwest::Error>{
        let url = &format!("{}/user_task_lists/{}/tasks?completed_since=now", Api::BASE_URL, self.user_gid);
        self.get(url)
    }   


    pub fn get_task(&self, task_id: &str) -> Result<models::Task, reqwest::Error> {
        let url = &format!("{}/tasks/{}", Api::BASE_URL, task_id);
        let response: HashMap<String, Value> = self.get(url)?;
        let data = match response.get("data") {
            Some(tmp) => tmp,
            None => panic!("[GET USER TASK] Task with id {} is not recognized", task_id)
        };
        let result: models::Task = serde_json::from_value(data.to_owned()).unwrap();
        return Ok(result)
    }

    pub fn update_task(&self, task_id: &str, body: HashMap<&str, Value>) -> Result<models::Task, reqwest::Error> {
        let url = &format!("{}/tasks/{}", Api::BASE_URL, task_id);
        let mut new_body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        new_body.insert("data", body);
        let response: HashMap<String, Value> = self.put(url, new_body)?;
        let data = match response.get("data") {
            Some(tmp) => tmp,
            None => panic!("[UPDATE TASK] Task with id {} is not recognized", task_id)
        };
        let result: models::Task = serde_json::from_value(data.clone()).unwrap();
        return Ok(result)
    }

    pub fn add_comment(&self, task_id: &str, comment: &str) -> Result<(), reqwest::Error> {
        let url = &format!("{}/tasks/{}/stories", Api::BASE_URL, task_id);
        let mut data_body: HashMap<&str, Value> = HashMap::new();
        data_body.insert("task", serde_json::to_value("1234").unwrap());
        data_body.insert("text", serde_json::to_value(comment).unwrap());
        let mut body: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        body.insert("data", data_body);
        let response: HashMap<String, Value> = self.post(url, body)?;
        match response.get("data") {
            Some(_) => (),
            None => panic!("[Add Comment] Task with id {} is not recognized", task_id)
        };
        return Ok(())
    }
}
