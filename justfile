website_run:
    cd resume && docker compose up --build jekyll

docusaurus_install:
    cd blog/website && npm install

blog_build:
    cd blog/website && npm run build

blog_run:
    cd blog/website && npm start

resume_build:
    cd resume && cargo run

resume_pdf:
    # TODO Should use a dedicaed image dockerfile to use correct user
    cd resume && docker compose up latex && sudo chown cdemonchy:cdemonchy resume.pdf
