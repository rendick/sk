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

fetch("/api/readme")
  .then((response) => response.json())
  .then((data) => {
    const readmeElement = document.querySelector(".readme");
    readmeElement.textContent = data.content;
  })
  .catch((err) => {
    console.error();
  });

fetch("/api/license")
  .then((response) => response.json())
  .then((data) => {
    const readmeElement = document.querySelector(".license");
    readmeElement.textContent = data.content;
  })
  .catch((err) => {
    console.error();
  });
