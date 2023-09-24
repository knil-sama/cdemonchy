module.exports={
  "title": "cdw technical blog",
  "tagline": "Technical article written by Clement Demonchy",
  "url": "https://cdemonchy.com/",
  "baseUrl": "/",
  "organizationName": "cdw",
  "projectName": "cdw",
  "scripts": [
    "https://buttons.github.io/buttons.js"
  ],
  "favicon": "data:image/x-icon;base64,AA",
  "onBrokenLinks": "log",
  "onBrokenMarkdownLinks": "log",
  "presets": [
    [
      "@docusaurus/preset-classic",
      {
        "theme": {
          "customCss": [require.resolve('./src/css/custom.css')],
        },
        "docs": false,
        "blog": {
          "path": "blog"
        },
      }
    ]
  ],
  "plugins": [],
  "themeConfig": {
    "navbar": {
      "title": "cdw technical blog",
      "logo": {
        "src": "TODO_CREATE_LOGO",
        "href": "blog"
      },
      "items": [],
    },
    "image": "img/undraw_online.svg",
    "footer": {
      "links": [],
      "copyright": "Copyright © 2023 cdw",
      "logo": {
        "src": "TODO_CREATE_LOGO",
        "alt": ""
      }
    }
  }
}