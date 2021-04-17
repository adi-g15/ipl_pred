const fetch = require("node-fetch");
const cheerio = require("cheerio");

function _original_client_side() {
    // execute while on "https://www.iplt20.com/matches/schedule/men"

	const matches = [];
	const date_nodes = document.querySelectorAll(".match-scroller__date");
	const nodes = document.querySelectorAll(".match-scroller__item");
	nodes.forEach((outer_node, i) => {
		const node = outer_node.querySelector(".match-item__content");
		let venue = outer_node.querySelector(".match-item__venue")
        if(venue)   venue = venue.innerText;
		let res = outer_node.querySelector(".match-item__summary")
        if(res)   res = res.innerText;
		matches.push({
			'0': node.querySelector('.match-item__team--a .match-item__team-name').innerText,
			'1': node.querySelector('.match-item__team--b .match-item__team-name').innerText,
			res,
			venue,
			date: (date_nodes[i]) ? [
				date_nodes[i].querySelector('.match-scroller__day').innerText,
				date_nodes[i].querySelector('.match-scroller__month').innerText
			].join(' '): null
		});
	});

	return matches;
}

async function scrollbar_way(event, context) {
    const html = await fetch("https://www.iplt20.com/matches/schedule/men")
                    .then(res => res.text())
    const $ = cheerio.load(html);

	const matches = [];
	const date_nodes = $(".match-scroller__date");
	const nodes = $(".match-scroller__item");

    const nodes_html = nodes.parent().html();
    return { statusCode: 200, body: nodes_html };

    nodes.map((i, outer_node) => {
	// nodes.forEach((outer_node, i) => {
        const {type, name, tagName, attribs} = outer_node
        console.log("Parent: ", tagName/*{type, name, tagName, attribs}*/)

        outer_node.childNodes.map(inner_node => {
            const {type, name, tagName} = inner_node
            // inner_node.
            if(type !== 'tag')  return;

            inner_node.

            console.log({type, name, tagName}, tagName ? inner_node.attribs.class: null )
        })
        return;
		const node = outer_node.querySelector(".match-item__content");
		let venue = outer_node.querySelector(".match-item__venue")
        if(venue)   venue = venue.innerText;
		let res = outer_node.querySelector(".match-item__summary")
        if(res)   res = res.innerText;
		matches.push({
			'0': node.querySelector('.match-item__team--a .match-item__team-name').innerText,
			'1': node.querySelector('.match-item__team--b .match-item__team-name').innerText,
			res,
			venue,
			date: (date_nodes[i]) ? [
				date_nodes[i].querySelector('.match-scroller__day').innerText,
				date_nodes[i].querySelector('.match-scroller__month').innerText
			].join(' '): null
		});
	});

	return {
        statusCode: 204
    };
}

const old_api = async (event, context) => {
    console.log("API hit");
    try{
        const html = await fetch("https://www.iplt20.com/matches/schedule/men")
                            .then(res => res.text())

        const $ = cheerio.load(html);
        const fixture_div_styles = ".fixture"
        const abbrev_team_name_styles = ".fixture__team-name--abbrv"

        let nodes = $(fixture_div_styles).toArray()

        const matches = nodes.map((_,node) => {
            const teams = []
            let d = node.children(abbrev_team_name_styles)
            d.forEach(team_node => {
                teams.push(team_node.innerText);
            })
            return {
                '0': teams[0],
                '1': teams[1],
                'time': time
            };

            // matches.push({
            //     '0': teams[0],
            //     '1': teams[1],
            //     'time': time
            // })
        })
        console.log(matches.get())

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

exports.handler = scrollbar_way;
