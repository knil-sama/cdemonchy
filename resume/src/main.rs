
mod askama_struct;
mod yaml_struct;
mod xml_struct;
pub mod naive_date_format;
pub use crate::xml_struct::{Resume};
pub use crate::yaml_struct::{Config, Sass, ContentWrapper, ContentDetail, AdditionalLink, LayoutDetail};
pub use crate::askama_struct::{LatexResume, Experience, Company};
use serde_xml_rs::from_reader;
use std::{fs::File};
use askama::Template; // bring trait in scope
use std::io::Write;
use regex::Regex;
use serde::Deserialize;
use thirtyfour::prelude::*;
use std::time::Duration;

fn parse_xml_file(xml_filepath: String) -> Resume {
    let file = File::open(xml_filepath).unwrap();
    from_reader(file).unwrap()
}

fn generate_yaml_config_from_xml_resume(xml_resume: Resume) {
    let sass: Sass = Sass{
        sass_dir: "_sass".to_string(),
        style: "compressed".to_string()
    };
    let content_projects: ContentWrapper = ContentWrapper{
        title: "Projects".to_string(),
        content: xml_resume.projects.into_iter()
        .map(|project| ContentDetail{
            title: project.title,
            layout: LayoutDetail::TopMiddle,
            description: highlight_skill(&project.description, &TypeHighlight::Markown),
            link: project.website.as_ref().map(|website| website.url.clone()),
            link_text: project.website.as_ref().map(|website| website.link_text.clone()),
            additional_links: project.github.as_ref().map(|github|
                vec![AdditionalLink{
                    title: github.link_text.clone(),
                    icon: "fab fa-github".to_string(),
                    url: github.url.clone(), 
                }]
            ),
            ..Default::default()
        }).collect(),
        ..Default::default()
    };
    let content_experiences: ContentWrapper = ContentWrapper{
        title: "Experiences".to_string(),
        content: xml_resume.experiences.into_iter()
        .map(|experience| ContentDetail{
            title: experience.company.name,
            link: Some(experience.company.url),
            quote: Some(experience.company.description),
            sub_title: Some(experience.role),
            caption: Some(format!("{} - {}", experience.start_date.format("%Y"), experience.end_date.format("%Y"))),
            description: highlight_skill(&experience.description, &TypeHighlight::Markown),
        ..Default::default()
        }).collect(),
    ..Default::default()
    };
    let content_education: ContentWrapper = ContentWrapper{
        title: "Education".to_string(),
        content: vec![ContentDetail{
            title: xml_resume.education.university,
            layout: LayoutDetail::TopLeft,
            sub_title: Some(xml_resume.education.title),
            caption: Some(format!("{} - {}", xml_resume.education.start_date.format("%Y"), xml_resume.education.end_date.format("%Y"))),
            description: xml_resume.education.description,
            ..Default::default()
        }],
        ..Default::default()
    };        
    let content: Vec<ContentWrapper> = vec![content_projects, content_experiences, content_education];
    let yaml_config: Config = Config{
        repository: "knil-sama/cdemonchy".to_string(),
        version: 2,
        name: format!("{} {}",xml_resume.personal.firstname, xml_resume.personal.lastname),
        title: xml_resume.personal.title,
        github_username: xml_resume.online.github_username,
        linkedin_username: xml_resume.online.linkedin_username,
        about_profile_image: xml_resume.personal.profile_pic,
        about_content: highlight_skill(&xml_resume.about_me, &TypeHighlight::Markown),
        content,
        footer_show_references: true,
        references_title: "References on request".to_string(),
        remote_theme: "sproogen/modern-resume-theme".to_string(),
        sass,
        plugins: vec!["jekyll-seo-tag".to_string()],
        exclude : vec![
          "Gemfile".to_string(),
          "Gemfile.lock".to_string(),
          "node_modules".to_string(),
          "vendor/bundle/".to_string(),
          "vendor/cache/".to_string(),
          "vendor/gems/".to_string(),
          "vendor/ruby/".to_string(),
          "lib/".to_string(),
          "scripts/".to_string(),
          "docker-compose.yml".to_string(),
          ],
        ..Default::default()
    };
    let output = File::create("./_config.yml").unwrap();
    let _ = serde_yaml_ng::to_writer(output,&yaml_config);
}
enum TypeHighlight {
    Latex,
    Markown
}
fn highlight_skill(content: &str, type_highlight: &TypeHighlight) -> String {
    let match_skill = Regex::new(r"(?<skill>\|REST|API|Github|Actions|Kafka|Spark|Pyspark|Airflow|Rust|TLA|Python|Scala|AWS|GCP|Azure|Databricks|Snowflake|Synapse|PostgreSQL|MongoDB|Neo4j|GKE|EC2|ECS|EMR|Cloudwatch|Lambda|S3|Redshift|Kubernetes|Argocd|Kopf|Debezium|Crossplane|Iceberg|Terraform|Flask|Aurora|Grafana|Ruff|Pydantic|Pytest)").unwrap();    
    match_skill.replace_all(content, match type_highlight {
        TypeHighlight::Latex => "\\textbf{$skill}",
        TypeHighlight::Markown => "**$skill**"
    }).to_string()
}

fn format_line_break(content: &str, type_highlight: &TypeHighlight) -> String {
    let match_skill = Regex::new(r"(?<line_break><br/>)").unwrap();
        match_skill.replace_all(content, match type_highlight {
        TypeHighlight::Latex => "\\\\",
        TypeHighlight::Markown => "<br/>"
    }).to_string()
}   


fn generate_latex_file(xml_resume: Resume, _rendered_template_filepath: String) {
    let latex_resume = LatexResume {
        firstname: xml_resume.personal.firstname,
        lastname: xml_resume.personal.lastname,
        email: xml_resume.personal.email,
        linkedin: xml_resume.online.linkedin_username,
        github: xml_resume.online.github_username,
        role: xml_resume.personal.title,
        portofolio_name: xml_resume.online.website,
        objective: format_line_break(&highlight_skill(&xml_resume.about_me, &TypeHighlight::Latex), &TypeHighlight::Latex),
        experiences: xml_resume.experiences.into_iter().map(|experience| askama_struct::Experience{ 
            company: askama_struct::Company {
                name: experience.company.name,
                url: experience.company.url,
                description: experience.company.description
            },
            role: experience.role,
            start_date: experience.start_date,
            end_date: experience.end_date,
            description: format_line_break(&highlight_skill(&experience.description, &TypeHighlight::Latex), &TypeHighlight::Latex),
            location: experience.location
        }).collect(),
        education: askama_struct::Education {
            title: xml_resume.education.title,
            university: xml_resume.education.university,
            description: xml_resume.education.description,
            start_date: xml_resume.education.start_date,
            end_date: xml_resume.education.end_date
        }
     }; // instantiate your struct
    let mut output = File::create("./resume.tex").unwrap();
    let _ = write!(output, "{}", latex_resume.render().unwrap());

}

async fn malt_update() -> WebDriverResult<()> {
    let config: TomlConfig = toml::from_str(&std::fs::read_to_string("src/.secrets.toml").unwrap()).unwrap();
    let caps = DesiredCapabilities::chrome();
    let server_url = "http://localhost:4444";
    let driver = WebDriver::new(server_url, caps).await?;
    driver.goto(format!("{}/signin", config.malt.url)).await?;
    assert_eq!("Connexion à Malt", driver.title().await?);
    let elem_form_email = driver.find(By::Id("email")).await?;
    let elem_form_password = driver.find(By::Id("password")).await?;
    elem_form_email.send_keys(config.malt.username).await?;
    elem_form_password.send_keys(config.malt.password).await?;
    // form.submit().await?;
    // println!("submitted ?");
    let elem_button = driver.find(By::Css("button[type='submit']")).await?;
    println!("found button");
    elem_button.click().await?;
    println!("after click");
    println!("Button disappeared ? {}", driver.query(By::Css("button[type='submit']")).wait(Duration::from_secs(10), Duration::from_secs(1)).not_exists().await?);

//     let form = driver.find(By::Id("signin-form")).await?;
//     println!("found form");
//     let error = form.find(By::ClassName("joy-form-error")).await?;
//     println!("{:?}", error);
//     while driver.title().await? == "Connexion à Malt" {
//         println!("waiting")
//     }
//     // while elem_button.is_present().await? {
//     //     println!("waiting");
//     // } // wait for signin page to be stale
//     println!("loaded ?");
//     driver.goto(format!("{}/profile", config.malt.url)).await?;
// //      // Look for header to implicitly wait for the page to load.
    println!("is logged ?");
    driver.goto(format!("{}/profile", config.malt.url)).await?;
    println!("{}", driver.title().await?);
    println!("{:?}",driver.find(By::ClassName("profile-headline-read-fullname")).await?.value().await.unwrap().ok_or("Not found"));
    
//      assert_eq!(driver.title().await?, "Selenium - Wikipedia");

//      // Always explicitly close the browser.
    driver.quit().await?;

   Ok(())
 }

#[derive(Deserialize)]
struct TomlConfig {
   malt: SiteConfig,
}

#[derive(Deserialize)]
struct SiteConfig {
   url: String,
   username: String,
   password: String,
}
#[tokio::main]
async fn main() {
    println!("start main");
    //let malt_updated = malt_update();
    //let _ = malt_updated.await;
    let xml_resume = parse_xml_file("src/resume.xml".to_string());
    generate_yaml_config_from_xml_resume(xml_resume.clone());
    generate_latex_file(xml_resume, "resume.tex".to_string());
    println!("end main");
}   
