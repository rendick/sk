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
