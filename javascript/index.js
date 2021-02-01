import {compile, newTask, deleteTask, toggleDependency, progressTask} from "../rust/Cargo.toml";
import Viz from "viz.js";
import { Module, render } from 'viz.js/full.render.js';
import "babel-polyfill";

const { remote } = nodeRequire('electron');

const viz = new Viz({ Module, render });

let code = "";

let remoteCode = remote.getGlobal("code");
if (remoteCode) {
  code = remoteCode;
}

console.log(code);

let codeElement = document.createElement("pre");
codeElement.textContent = code;

function updateCode(newCode) {
  code = newCode;
  renderGraph();
  codeElement.textContent = code;

  const save = remote.getGlobal("save");
  if (save) {
    save(code);
  }
}

let input = document.createElement("textarea");
input.addEventListener("keypress", function (e) {
  if (e.keyCode == 13) {
    e.preventDefault();
    let text = e.srcElement.value.trim();
    if (text.length != 0) {
      if (text.includes("-") || text.includes(">") || text.includes("\n")) {
        updateCode(text);
      } else {
        updateCode(newTask(text, code));
      }
    }

    e.srcElement.value = "";
  }
});

let svgContainer = document.createElement("div");
let tmpNode = document.createElement("tmp");
svgContainer.appendChild(tmpNode);

function instrumentNodes(svgElement) {
  let dragFrom = null;
  for (let node of svgElement.querySelectorAll(".node")) {
    let identifier = node.querySelector("text").textContent.trim();
    node.addEventListener("mousedown", function (e) {
      if (e.which == 2) {
        e.preventDefault();
        updateCode(deleteTask(identifier, code));
      } else if (e.which == 1) {
        e.preventDefault();
        dragFrom = identifier;
      }
    });

    node.addEventListener("mouseup", function (e) {
      if (e.which == 1 && dragFrom != null) {
        if (identifier === dragFrom) {
          updateCode(progressTask(identifier, code));
        } else {
          updateCode(toggleDependency(identifier, dragFrom, code));
        }
        dragFrom = null;
      }
    });
  }
}

async function renderGraph() {
  let compileResult = compile(code);
  if (compileResult.success) {
    let element = await viz.renderSVGElement(compileResult.dotCode);
    instrumentNodes(element);
    svgContainer.replaceChild(element, svgContainer.children[0]);
  } else {
    console.error(compileResult.reason);
  }
}

document.body.appendChild(input);
document.body.appendChild(svgContainer);
document.body.appendChild(codeElement);

document.addEventListener("keydown", async function (e) {
  if (e.keyCode == 83 && e.ctrlKey) {
    e.preventDefault();

    const saveAs = remote.getGlobal("saveAs");
    if (saveAs) {
      saveAs(code);
    }
  }
});

renderGraph();
