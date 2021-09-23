import { readFileSync, writeFileSync } from "fs";
import path from "path";
import {fileURLToPath} from "url";

// Using __dirname in ES modules: https://stackoverflow.com/a/69242626/12339402
const json_path = path.join( path.dirname(fileURLToPath(import.meta.url)), "graph_data.json" );
const json = JSON.parse( readFileSync(json_path) );
const keys = Object.keys(json);

console.log("Prettifying...");

try {
    keys.sort((a, b) => {
        let a_int = 0;
        let b_int = 0;

        try {
            a_int = parseInt(a);
            b_int = parseInt(b);
        } catch { }

        if (a == "msg") {
            return -Infinity;
        } else if (b == "msg") {
            return Infinity;
        } else if (a.includes("_msg")) {
            if (b.includes("_msg")) {
                return parseInt(a.substr(0, a.indexOf("_msg"))) - parseInt(b.substr(0, b.indexOf("_msg")));
            } else {
                return parseInt(a.substr(0, a.indexOf("_msg"))) - b_int - 1;
            }
        } else if (b.includes("_msg")) {
            return a_int - parseInt(b.substr(0, b.indexOf("_msg")));
        } else {
            return a_int - b_int;
        }
    });
} catch { keys.sort(); }

const output = {};

for (let key of keys) {
    output[key] = json[key];
}

writeFileSync(json_path, JSON.stringify(output, null, 4));
