# cdemonchy
main access to public online project

## CV

Template from https://github.com/sproogen/modern-resume-theme
deployed on https://cdemonchy.com/

### local run

```bash
cd resume
bundle install
bundle exec jekyll serve
```

Open your browser to http://localhost:4000

## Blog

### Run docusaurus locally

```bash
cd blog/website
npm start
```

### Build docusaurus

```bash
cd blog/website
npm run build
```

# Workflow

```mermaid
flowchart LR

A[resume.xml] --> B{Rust parser}
B --> C[_config.yaml] 
C --> J{Jekyll} 
J --> S[index.html] 
B --> L[resume.tex] --> P[resume.pdf] 
B --> t[TODO] --> Marketplace{shape: documents }
```

# TODO

[x] Convert yaml to xml file
[] Validate XML
[] Ask for correction on english
[] read and parse using serde-xml-rs
[] yaml for jenkins
[] latex -> pdf
[] update cv on other plateforms