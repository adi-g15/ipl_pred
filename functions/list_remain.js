const fetch = require("node-fetch");
const $ = require("cheerio");

function way_2() {
	const matches = [];
	const document = $.load("");
	const date_nodes = document.querySelectorAll(".match-scroller__date");
	const nodes = document.querySelectorAll(".match-scroller__item");
	nodes.forEach((outer_node, i) => {
		const node = outer_node.querySelector(".match-item__content");
		const venue = outer_node.querySelector(".match-item__venue")
		const res = outer_node.querySelector(".match-item__summary")
		matches.push({
			'0': node.querySelector('.match-item__team--a .match-item__team-name').innerText,
			'1': node.querySelector('.match-item__team--b .match-item__team-name').innerText,
			res,
			venue,
			date: [
				date_nodes[i].querySelector('.match-scroller__day').innerText
				date_nodes[i].querySelector('.match-scroller__month').innerText
			]
		});
	});

	return matches;
}

exports.handler = async (event, context) => {
    try{
        const html = await fetch("https://www.iplt20.com/matches/schedule/men")
                            .then(res => res.text())

        const document = $.load(html);
        const fixture_div_styles = ".fixture"
        const abbrev_team_name_styles = ".fixture__team-name--abbrv"

        let nodes = document.querySelectorAll(fixture_div_styles)

        const matches = []
        
        nodes.forEach(node => {
            const teams = []
            node.querySelectorAll(abbrev_team_name_styles).forEach(team_node => {
                teams.push(team_node.innerText);
            })
            matches.push({
                '0': teams[0],
                '1': teams[1],
                'time': time
            })
        })
        
        return {
            statusCode: 200,
            body: JSON.stringify(matches)
        };
    } catch (err){
        return {
            statusCode: 500,
            body: JSON.stringify(err.message || "Internal Server Error")
        }
    }
}
