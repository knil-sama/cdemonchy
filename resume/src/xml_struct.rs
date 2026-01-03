use serde::Deserialize;
use url::{Url};
use chrono::NaiveDate;


// from https://dev.to/thiagomg/another-way-to-deserialise-datetime-in-rust-kja
mod naive_date_format {  
    use chrono::NaiveDate;  
    use serde::{self, Deserialize, Deserializer, Serializer};  

    const FORMAT: &str = "%Y-%m-%d";  

    /// Transforms a NaiveDate into a String
    #[allow(dead_code)]
    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {  
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    /// Transforms a String into a NaiveDate
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>  
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Resume {
    pub personal: Personal,
    pub online: Online,
    pub about_me: String,
    pub projects: Projects,
    pub experiences: Experiences,
    pub education: Education
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Personal {
    pub firstname: String,
    pub lastname: String,
    pub title: String,
    pub profile_pic: String
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Online {
    pub website: String,
    pub github_username: String,
    pub linkedin_username: String
}

#[derive(Debug, PartialEq, Deserialize)]
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

#[derive(Debug, PartialEq, Deserialize)]
pub struct Project {
    pub title: String,
    pub website: Option<Website>,
    pub github: Option<Github>,
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Website {
    pub url: Url,
    pub link_text: String
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Github {
    pub url: Url,
    pub link_text: String
}

#[derive(Debug, PartialEq, Deserialize)]
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

#[derive(Debug, PartialEq, Deserialize)]
pub struct Experience {
    pub company: Company,
    pub role: String,
    #[serde(with = "naive_date_format")]
    pub start_date: NaiveDate,
    #[serde(with = "naive_date_format")]
    pub end_date: NaiveDate,
    pub description: String
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Company {
    pub name: String,
    pub url: Url,
    pub description: String
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Education {
    pub title: String,
    pub university: String,
    pub description: String,
    #[serde(with = "naive_date_format")]
    pub start_date: NaiveDate,
    #[serde(with = "naive_date_format")]
    pub end_date: NaiveDate
}
