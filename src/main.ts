import { invoke } from "@tauri-apps/api/tauri";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

let filePath: HTMLInputElement | null;
let filePathResult: HTMLElement | null;

async function fileExists() {
  if (filePath && filePathResult) {
    filePathResult.textContent = await invoke("file_exists", {

      file: filePath.value
    }) ? "yes" : "no"
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  filePath = document.querySelector("#file-input");
  filePathResult = document.querySelector("#file-result")

  document.querySelector("#file-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    fileExists();
  });

});
