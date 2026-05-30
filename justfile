website_run:
    cd resume && docker compose up --build jekyll

docusaurus_install:
    cd blog/website && npm install

blog_build:
    cd blog/website && npm run build

blog_run:
    cd blog/website && npm start
