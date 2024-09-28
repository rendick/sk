const fs = require("fs");
const path = require("path");
const express = require("express");
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

app.get("/config", (req, res) => {
  const logFilePath = path.join(directoryPath, ".sk", "config");

  let content = "No config file found!";

  if (fs.existsSync(logFilePath)) {
    content = fs.readFileSync(logFilePath, "utf8");
  }

  fs.readFile(path.join(__dirname, "config.html"), "utf8", (err, html) => {
    if (err) {
      res.status(500).send("sdjfsd");
      return;
    }

    const responseHtml = html.replace("${content}", content);
    res.send(responseHtml);
  });
});

app.get("/commits", (req, res) => {
  const commitFilePath = path.join(directoryPath, ".sk", "logs");

  let content = "No commit file found!";

  if (fs.existsSync(commitFilePath)) {
    content = fs.readFileSync(commitFilePath, "utf8");
  }
  fs.readFile(path.join(__dirname, "commits.html"), "utf8", (err, html) => {
    if (err) {
      res.status(500).send("sdfdf");
    }

    const responseHtml = html.replace("${content}", content);
    res.send(responseHtml);
  });
});

app.get("/api/readme", (req, res) => {
  const readmePath = path.join(directoryPath, "README.md");

  if (fs.existsSync(readmePath)) {
    const content = fs.readFileSync(readmePath, "utf8");
    res.json({ content });
  } else {
    res.status(404).join({ content: "README.md file not found." });
  }
});

app.get("/api/license", (req, res) => {
  const readmePath = path.join(directoryPath, "LICENSE");

  if (fs.existsSync(readmePath)) {
    const content = fs.readFileSync(readmePath, "utf8");
    res.json({ content });
  } else {
    res.status(404).join({ content: "README.md file not found." });
  }
});

app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});
