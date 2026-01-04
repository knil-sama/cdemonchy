use serde::Deserialize;
use url::{Url};
use chrono::NaiveDate;
use serde_email::Email;
pub use crate::naive_date_format;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Resume {
    pub personal: Personal,
    pub online: Online,
    pub about_me: String,
    pub projects: Projects,
    pub experiences: Experiences,
    pub education: Education
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Personal {
    pub firstname: String,
    pub lastname: String,
    pub title: String,
    pub profile_pic: String,
    pub email: Email
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Online {
    pub website: String,
    pub github_username: String,
    pub linkedin_username: String
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Projects {
    // see this issue on why we need intermediate structure https://github.com/RReverser/serde-xml-rs/issues/243
    #[serde(default, rename = "project")]
    pub projects: Vec<Project>
}

// and we'll implement IntoIterator
impl IntoIterator for Projects {
    type Item = Project;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.projects.into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Project {
    pub title: String,
    pub website: Option<Website>,
    pub github: Option<Github>,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Website {
    pub url: Url,
    pub link_text: String
}


#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Github {
    pub url: Url,
    pub link_text: String
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Experiences {
    // see this issue on why we need intermediate structure https://github.com/RReverser/serde-xml-rs/issues/243
    #[serde(default, rename = "experience")]
    pub experiences: Vec<Experience>
}

// and we'll implement IntoIterator
impl IntoIterator for Experiences {
    type Item = Experience;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.experiences.into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Experience {
    pub company: Company,
    pub role: String,
    #[serde(with = "naive_date_format")]
    pub start_date: NaiveDate,
    #[serde(with = "naive_date_format")]
    pub end_date: NaiveDate,
    pub description: String,
    pub location: String
}


#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Company {
    pub name: String,
    pub url: Url,
    pub description: String
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Education {
    pub title: String,
    pub university: String,
    pub description: String,
    #[serde(with = "naive_date_format")]
    pub start_date: NaiveDate,
    #[serde(with = "naive_date_format")]
    pub end_date: NaiveDate
}
