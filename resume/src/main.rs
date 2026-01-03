mod xml_struct;
mod yaml_struct;

pub use crate::xml_struct::{Resume};
pub use crate::yaml_struct::{Config, Sass, ContentWrapper, ContentDetail, AdditionalLink, LayoutDetail};
use serde_xml_rs::from_reader;
use std::{fs::File};


fn main() {
    let file = File::open("src/resume.xml").unwrap();
    let xml_resume: Resume = from_reader(file).unwrap();
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
            description: project.description,
            link: match project.website {
                Some(ref website) => Some(website.url.clone()),
                _ => None
            },
            link_text:  match project.website {
                Some(ref website) => Some(website.link_text.clone()),
                _ => None 
            },
            additional_links: match project.github {
                Some(github) => Some(vec![AdditionalLink{
                    title: github.link_text,
                    icon: "fab fa-github".to_string(),
                    url: github.url, 
                    }]),
                _ => None
            },
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
            caption: Some(experience.start_date.format("%Y").to_string() + " - " + &experience.end_date.format("%Y").to_string()),
            description: experience.description,
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
            caption: Some(xml_resume.education.start_date.format("%Y").to_string() + " - " + &xml_resume.education.end_date.format("%Y").to_string()),
            description: xml_resume.education.description,
            ..Default::default()
        }],
        ..Default::default()
    };        
    let content: Vec<ContentWrapper> = vec![content_projects, content_experiences, content_education];
    let yaml_config: Config = Config{
        repository: "knil-sama/cdemonchy".to_string(),
        version: 2,
        name: xml_resume.personal.firstname + " " + &xml_resume.personal.lastname,
        title: xml_resume.personal.title,
        github_username: xml_resume.online.github_username,
        linkedin_username: xml_resume.online.linkedin_username,
        about_profile_image: xml_resume.personal.profile_pic,
        about_content: xml_resume.about_me,
        content: content,
        footer_show_references: true,
        references_title: "References on request".to_string(),
        remote_theme: "sproogen/modern-resume-theme".to_string(),
        sass: sass,
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
