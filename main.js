const { app, BrowserWindow, dialog } = require('electron');
const fs = require('fs');

let path = null;
if (process.argv[1] && fs.existsSync(process.argv[1])) {
  path = process.argv[1];
  console.log(path);
  global.code = fs.readFileSync(path, "utf8");
}

global.save = (code) => {
  fs.writeFileSync(path, code);
};


function createWindow() {
  let win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: true
    }
  });

  global.saveAs = (code) => {
    path = dialog.showSaveDialogSync(win);
    global.save(code);
  };

  win.loadFile("./dist/index.html");
}

app.whenReady().then(createWindow);
