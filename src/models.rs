use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;


#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Resource {
    pub gid: String,
    pub name: String,
    pub resource_type: String,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} Id: {}",
            self.resource_type.to_uppercase(),
            self.name.trim(),
            self.gid
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Task {
    pub assignee: Option<Resource>,
    assignee_status: Option<String>,
    pub completed: bool,
    completed_at: Option<String>,
    created_at: String,
    due_at: Option<String>,
    due_on: Option<String>,
    followers: Vec<Resource>,
    pub gid: String,
    hearted: bool,
    hearts: Vec<Resource>,
    liked: bool,
    likes: Vec<Resource>,
    memberships: Vec<HashMap<String, Resource>>,
    modified_at: String,
    pub name: String,
    pub notes: String,
    num_hearts: u32,
    num_likes: u32,
    parent: Option<Resource>,
    projects: Vec<Resource>,
    resource_subtype: String,
    resource_type: String,
    start_on: Option<String>,
    tags: Vec<Resource>,
    workspace: Resource,
}

impl Task {
    pub fn title(&self) -> &str {
        match &self.completed {
            true => "[COMPLETED] Tache",
            false => "Tache",
        }
    }

    pub fn assignee_name(&self) -> &str {
        match &self.assignee {
            None => "Non Renseigné",
            Some(tmp) => tmp.name.as_str(),
        }
    }

    pub fn due_date(&self) -> &str {
        match &self.due_at {
            None => "Non défini",
            Some(tmp) => tmp,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\nId: {}\nCreated at {}\nAssigned to {}\nDue Date : {}\nInformations : {}\n",
            self.title(),
            self.gid,
            self.name,
            self.created_at,
            self.assignee_name(),
            self.due_date(),
            self.notes
        )
    }
}


#[derive(Deserialize, Debug)]
pub struct Tasks {
    pub data: Vec<Resource>
}

impl fmt::Display for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.iter().fold(Ok(()), |result, ws | {
            result.and_then(|_| writeln!(f, "{}", ws))
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct Workspaces {
    pub data: Vec<Resource>
}

impl fmt::Display for Workspaces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.iter().fold(Ok(()), |result, ws | {
            result.and_then(|_| writeln!(f, "{}", ws))
        })
    }
}


#[derive(Deserialize, Debug)]
pub struct Projects {
    pub data: Vec<Resource>
}

impl fmt::Display for Projects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.iter().fold(Ok(()), |result, ws | {
            result.and_then(|_| writeln!(f, "{}", ws))
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct Users {
    pub data: Vec<Resource>
}

impl fmt::Display for Users {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.iter().fold(Ok(()), |result, ws | {
            result.and_then(|_| writeln!(f, "{}", ws))
        })
    }
}