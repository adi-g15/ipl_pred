const fetch = require("node-fetch");
const $ = require("cheerio");

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
