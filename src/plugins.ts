import { invoke } from "@tauri-apps/api/tauri";
import { Plugin } from "../src-tauri/bindings/Plugin.ts";
// import { PluginType } from "../src-tauri/bindings/PluginType.ts";

let pluginTable: HTMLTableElement | null;
// let plugins: Array<Plugin>;
// async function findPlugins() {
//     if (filePath && filePathResult) {
//         plugins = await invoke("plugins", {})
//     }
// }

async function populateTable() {
    let plugins: Array<Plugin> = await invoke("plugins", {})
    console.log(plugins)
    if (pluginTable) {
        console.log("hello")
        plugins.forEach((plugin) => {

            // @ts-ignore
            const row = pluginTable.insertRow();
            let cells = [plugin.plugin_type, plugin.name, plugin.vendor, plugin.path_to_file]

            cells.forEach((cellData) => {
                const cell = row.insertCell()
                cell.innerHTML = cellData === null ? "" : cellData
            })
        })
    }
}

window.addEventListener("DOMContentLoaded", () => {

    // filePath = document.querySelector("#file-input");
    // filePathResult = document.querySelector("#file-result")
    pluginTable = document.querySelector("#plugin-table")
    populateTable()

    // document.querySelector("#file-form")?.addEventListener("submit", (e) => {
    //     e.preventDefault();
    // });

});
