use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CV {
    pub title: String,
    pub subtitle: String,
    pub identity: Identity,
    pub contact: Contact,
    pub skills: Vec<Skill>,
    pub degrees: Vec<Degree>,
    pub experiences: Vec<Experience>,
    pub personal_projects: Option<Vec<PersonalProject>>
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub firstname: String,
    pub lastname : String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub phone: String,
    pub email: String,
    pub address: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    pub domain: String,
    pub items: Vec<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Degree {
    pub start_year: u32,
    pub end_year: u32,
    pub label: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Experience {
    pub city: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub highlights: Vec<String>,
    pub technical_stack: Vec<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalProject {
    pub project_name: String,
    pub description: String,
    pub backend_stack : Vec<String>,
    pub frontend_stack : Vec<String>,
    pub integration_stack : Vec<String>
}


impl CV {
    pub fn create(path: String) -> anyhow::Result<CV> {
        let result = std::fs::read_to_string(path)?;
        let cv: CV = serde_yaml::from_str(&result)?;
        Ok(cv)
    }
}