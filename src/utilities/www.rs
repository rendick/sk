// back-end

pub const INDEX: &str = r#"
const fs = require("fs");
const path = require("path");
const express = require("express");
const toml = require("toml");
const app = express();
const PORT = 3000;
let files = [];

const directoryPath = "./repo";

app.use(express.static("public"));

function readDir(ditPath) {
  let fileList = [];
  const items = fs.readdirSync(ditPath, { withFileTypes: true });
  items.forEach((item) => {
    const fullPath = path.join(ditPath, item.name);
    if (item.isDirectory()) {
      fileList = fileList.concat(readDir(fullPath));
    } else {
      fileList.push(fullPath);
    }
  });
  return fileList;
}

try {
  files = readDir(directoryPath);
  console.log(files);
} catch (err) {
  console.error(err);
}

app.get("/api/files", (req, res) => {
  res.json(files);
});

app.get("/file", (req, res) => {
  const filePath = req.query.path;

  if (fs.existsSync(filePath)) {
    const content = fs.readFileSync(filePath, "utf8");
    res.send(`<pre>${content}</pre>`);
  } else {
    res.status(404).send("File not found");
  }
});

function pageOptimize(dir, file_link,  file, errormsg) {
  app.get(`/${file_link}`, (req, res) => {
    const logFilePath = path.join(directoryPath, dir, file);

    let content = errormsg;

    if (fs.existsSync(logFilePath)) {
      content = fs.readFileSync(logFilePath, "utf8");
    }

    fs.readFile(
      path.join(__dirname, `../../public/${file_link}.html`),
      "utf8",
      (err, html) => {
        if (err) {
          res.status(500).send("Internal server error");
          return;
        }

        const responseHtml = html.replace("${content}", content);
        res.send(responseHtml);
      }
    );
  });
}

pageOptimize(".sk", "config", "config", "Config file not found.");
pageOptimize(".sk", "commits", "commits", "Commit file not found.");

function apiOptimize(routePath, file, errormsg) {
  app.get(`/api/${routePath}`, (req, res) => {
    const readmePath = path.join(directoryPath, file);

    if (fs.existsSync(readmePath)) {
      const content = fs.readFileSync(readmePath, "utf8");
      res.json({ content });
    } else {
      res.status(404).json({ content: errormsg });
    }
  });
}

apiOptimize("readme", "README.md", "README.md file not found.");
apiOptimize("license", "LICENSE", "LICENSE file not found.");

app.get("/api/name", (req, res) => {
  const namePath = path.join(directoryPath, ".sk", "config");

  if (fs.existsSync(namePath)) {
    const content = fs.readFileSync(namePath, "utf8");
    try {
      const config = toml.parse(content);
      const projectName = config.project?.name || "Unnamed Project";
      res.json({ name: projectName });
    } catch (err) {
      res.status(500).json({ error: "Failed to parse TOML file." });
    }
  } else {
    res.status(404).json({ content: "Config file not found." });
  }
});

app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});
"#;

pub const INDEX_PATH: &str = "./public/js/index.js";

pub const FETCH: &str = r#"
fetch("/api/files")
  .then((response) => response.json())
  .then((files) => {
    const fileList = document.getElementById("file-list");

    files.forEach((file) => {
      const listItem = document.createElement("li");
      const link = document.createElement("a");

      link.href = `/file?path=${encodeURIComponent(file)}`;
      link.textContent = file;
      listItem.appendChild(link);
      fileList.appendChild(listItem);
    });
  })
  .catch((err) => {
    console.error("Error fetching files:", err);
  });

function fetchOptimize(apiname, selector) {
  fetch(`/api/${apiname}`)
    .then((response) => response.json())
    .then((data) => {
      const readmeElement = document.querySelector(selector);
      readmeElement.textContent = data.content;
    })
    .catch((err) => {
      console.error();
    });
}

fetchOptimize("readme", ".readme");
fetchOptimize("license", ".license");
fetchOptimize("name", ".names");
"#;
pub const FETCH_PATH: &str = "./public/js/fetch.js";

// front-end

pub const MAIN: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="./style/style.css" />
    <title class="names"></title>
    <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
  </head>
  <body>
    <div class="header">
      <center>
        <div class="header-center">
          <a class="active" href="/">Repository</a>
          <a href="/commits">Commits</a>
          <a href="/config">Config</a>
        </div>
      </center>
    </div>
    <div class="container">
      <div class="files">
        <h1>
          <a href="/api/files" style="text-decoration: none; font-size: 35px"
            >Files</a
          >
        </h1>
        <ul id="file-list"></ul>
      </div>
      <div class="description">
        <div class="README">
          <p>
            <b
              ><a
                href="/api/readme"
                style="text-decoration: none; font-size: 25px"
                >README.md</a
              ></b
            >
          </p>
          <pre
            class="readme"
            style="font-family: 'Courier New', Courier, monospace"
          ></pre>
        </div>
        <hr />
        <div class="LICENSE">
          <div class="cont">
            <p>
              <b
                ><a
                  href="/api/license"
                  style="text-decoration: none; font-size: 25px"
                  >LICENSE</a
                ></b
              >
            </p>
          </div>
          <pre
            class="license"
            style="font-family: 'Courier New', Courier, monospace"
          ></pre>
        </div>
      </div>
    </div>
    <script src="./js/fetch.js"></script>
  </body>
</html>
"#;
pub const MAIN_PATH: &str = "./public/index.html";

pub const COMMIT: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="./style/style.css" />
    <title>commits</title>
  </head>
  <body>
    <div class="header">
      <center>
        <div class="header-center">
          <a href="/">Repository</a>
          <a class="active" href="/commits">Commits</a>
          <a href="/config">Config</a>
        </div>
      </center>
    </div>
    <div class="container">
      <pre>${content}</pre>
    </div>
  </body>
</html>
"#;
pub const COMMIT_PATH: &str  = "./public/commits.html";

pub const CONFIG: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="./style/style.css" />
    <title>config</title>
  </head>
  <body>
    <div class="header">
      <center>
        <div class="header-center">
          <a href="/">Repository</a>
          <a href="/commits">Commits</a>
          <a class="active" href="/config">Config</a>
        </div>
      </center>
    </div>
    <div class="container">
      <pre>${content}</pre>
    </div>
  </body>
</html>
"#;

pub const CONFIG_DIR: &str = "./public/config.html";

pub const STYLE_CSS: &str = r#"body {
  margin: 0 auto;
  background-color: #111111;
  color: #ffffff;
  font-family: "Courier New", Courier, monospace;
}

.container {
  max-width: 1300px;
  margin: 0 auto;
}

a {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  color: #ffffff;
}

.header {
  overflow: hidden;
  background-color: #111111;
}

.header a {
  float: left;
  color: #90ee90;
  text-align: center;
  padding: 10px;
  text-decoration: none;
  font-size: 18px;
  line-height: 25px;
}

.header a.logo {
  font-size: 25px;
  font-weight: bold;
}

.header a:hover {
  background-color: #ff1493;
  color: #ffffff;
}

.header a.active {
  background-color: #ff1493;
  color: #ffffff;
}

.header-center {
  display: flex;
  justify-content: center;
  align-items: center;
}

@media screen and (max-width: 500px) {
  .header a {
    float: none;
    display: block;
    text-align: left;
  }
  .header-right {
    float: none;
  }
}

.description {
  background-color: #1a1919;
  padding: 5px;
  margin-top: 10px;
}

.files {
  background-color: #1a1919;
  padding: 5px;
  margin-top: 10px;
}
"#;

pub const STYLE_PATH: &str = "./public/style/style.css";

pub const JSON_CONFIG: &str = r#"{
  "name": "N/A",
  "version": "x.x.x",
  "description": "N/A",
  "main": "public/js/index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1",
    "start": "node public/js/index.js"
  },
  "repository": {
    "url": "N/A"
  },
  "dependencies": {
    "express": "^4.21.0",
    "marked": "^14.1.2",
    "toml": "^3.0.0"
  },
  "author": "N/A",
  "license": "N/A"
}
"#;
pub const JSON_CONFIG_PATH: &str = "./package.json";
