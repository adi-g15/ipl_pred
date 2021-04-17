const fetch = require("node-fetch");
const HTMLParser = require("fast-html-parser");
const { writeFileSync, existsSync } = require("fs");

fetch("https://www.iplt20.com/matches/schedule/men")
    .then(res => res.text())
    .then(html => {
        const document = HTMLParser.parse(html);

        const matches = [];
        const date_nodes = document.querySelectorAll(".match-scroller__date");
        const nodes = document.querySelectorAll(".match-scroller__item");
        nodes.forEach((outer_node, i) => {
            const node = outer_node.querySelector(".match-item__content");
            let venue = outer_node.querySelector(".match-item__venue")
            if (venue) {
                venue = venue.text.trim();
            }
            let res = outer_node.querySelector(".match-item__summary")
            if (res) res = res.text.trim();

            matches.push({
                '0': node.querySelector('.match-item__team--a .match-item__team-name').text.trim(),
                '1': node.querySelector('.match-item__team--b .match-item__team-name').text.trim(),
                res,
                venue,
                date: (date_nodes[i]) ? [
                    date_nodes[i].querySelector('.match-scroller__day').text.trim(),
                    date_nodes[i].querySelector('.match-scroller__month').text.trim()
                ].join(' ') : null
            });
        });

        return matches
    })
    .then(matches => {
        if(existsSync("data/"))
            writeFileSync("data/matches.json", JSON.stringify(matches, null, 4));
        else
            writeFileSync("matches.json", JSON.stringify(matches, null, 4));

        console.log("Updated data/matches.json");
    })
    .catch(err => {
        console.error(err);
    })
