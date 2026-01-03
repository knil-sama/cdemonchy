use serde::{Serialize, Deserialize};
use url::{Url};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub baseurl: String,
    pub repository: String,
    pub version: i32,
    pub name: String,
    pub title: String,
    pub github_username: String,
    pub linkedin_username: String,
    #[serde(default)] // False
    pub darkmode: bool,
    pub about_profile_image: String,
    pub about_content: String,
    pub content: Vec<ContentWrapper>,
    pub footer_show_references: bool,
    pub references_title: String,
    pub remote_theme: String,
    pub sass: Sass,
    pub plugins: Vec<String>,
    pub exclude : Vec<String>
}


#[derive(Default, Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutWrapper { 
    #[default]
    List, 
    Text
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContentWrapper {
    pub title: String,
    #[serde(default)] // List
    pub layout: LayoutWrapper,
    pub content: Vec<ContentDetail>,
}



#[derive(Default, Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutDetail { 
    #[default]
    Left, 
    TopLeft, 
    TopMiddle}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContentDetail {
    pub title: String,
    pub layout: LayoutDetail,
    pub link: Option<Url>,
    pub link_text: Option<String>,
    pub additional_links: Option<Vec<AdditionalLink>>,
    pub sub_title: Option<String>,
    pub quote: Option<String>,
    pub caption: Option<String>,
    pub description: String,
    pub name: Option<String>
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Sass {
    pub sass_dir: String,
    pub style: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalLink {
    pub title: String,
    pub icon: String,
    pub url: Url
}
