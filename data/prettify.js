const json = require("./graph_data.json");
const { writeFileSync } = require("fs");

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

// writeFileSync("graph_data.json", JSON.stringify(json, null, 4));
writeFileSync("graph_data.json", JSON.stringify(output, null, 4));
