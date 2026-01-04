use askama::Template; // bring trait in scope
use serde_email::Email;
use serde::Deserialize;
use url::{Url};
use chrono::NaiveDate;
pub use crate::naive_date_format;

#[derive(Template)] // this will generate the code...
#[template(path = "resume.tex")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
pub struct LatexResume { // the name of the struct can be anything
    pub firstname: String, // the field name should match the variable name
    pub lastname: String,               // in your template
    pub email: Email,
    pub linkedin: String,
    pub github: String,
    pub role: String,
    pub portofolio_name: String,
    pub objective: String,
    pub experiences: Vec<Experience>,
    pub education: Education
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