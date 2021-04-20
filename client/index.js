import * as wasm from "../pkg";

const matches_str = `[{"0":"MI","1":"RCB","res":"RCB won by 2 wickets","venue":null,"date":"9 Apr"},{"0":"CSK","1":"DC","res":"DC won by 7 wickets","venue":null,"date":"10 Apr"},{"0":"SRH","1":"KKR","res":"KKR won by 10 runs","venue":null,"date":"11 Apr"},{"0":"RR","1":"PBKS","res":"PBKS won by 4 runs","venue":null,"date":"12 Apr"},{"0":"KKR","1":"MI","res":"MI won by 10 runs","venue":null,"date":"13 Apr"},{"0":"SRH","1":"RCB","res":"RCB won by 6 runs","venue":null,"date":"14 Apr"},{"0":"RR","1":"DC","res":"RR won by 3 wickets","venue":null,"date":"15 Apr"},{"0":"PBKS","1":"CSK","res":"CSK won by 6 wickets","venue":null,"date":"16 Apr"},{"0":"MI","1":"SRH","res":"MI won by 13 runs","venue":null,"date":"17 Apr"},{"0":"RCB","1":"KKR","res":"RCB won by 38 runs","venue":null,"date":"18 Apr"},{"0":"DC","1":"PBKS","res":"DC won by 6 wickets","venue":null,"date":"19 Apr"},{"0":"CSK","1":"RR","res":null,"venue":"Wankhede Stadium, Mumbai","date":"20 Apr"},{"0":"DC","1":"MI","res":null,"venue":"M. A. Chidambaram Stadium, Chennai","date":"21 Apr"},{"0":"PBKS","1":"SRH","res":null,"venue":"M. A. Chidambaram Stadium, Chennai","date":"22 Apr"},{"0":"KKR","1":"CSK","res":null,"venue":"Wankhede Stadium, Mumbai","date":"23 Apr"},{"0":"RCB","1":"RR","res":null,"venue":"Wankhede Stadium, Mumbai","date":"24 Apr"},{"0":"PBKS","1":"MI","res":null,"venue":"M. A. Chidambaram Stadium, Chennai","date":"25 Apr"},{"0":"RR","1":"KKR","res":null,"venue":"Wankhede Stadium, Mumbai","date":"26 Apr"},{"0":"CSK","1":"RCB","res":null,"venue":"Wankhede Stadium, Mumbai","date":"27 Apr"},{"0":"SRH","1":"DC","res":null,"venue":"M. A. Chidambaram Stadium, Chennai","date":"28 Apr"},{"0":"PBKS","1":"KKR","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"29 Apr"},{"0":"DC","1":"RCB","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"30 Apr"},{"0":"CSK","1":"SRH","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"1 May"},{"0":"MI","1":"RR","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"2 May"},{"0":"DC","1":"KKR","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"3 May"},{"0":"PBKS","1":"RCB","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"4 May"},{"0":"MI","1":"CSK","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"5 May"},{"0":"RR","1":"SRH","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"6 May"},{"0":"PBKS","1":"DC","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"7 May"},{"0":"KKR","1":"RCB","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"8 May"},{"0":"SRH","1":"MI","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"9 May"},{"0":"RR","1":"CSK","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"10 May"},{"0":"RCB","1":"PBKS","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"11 May"},{"0":"SRH","1":"CSK","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"12 May"},{"0":"KKR","1":"DC","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"13 May"},{"0":"RR","1":"MI","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"14 May"},{"0":"CSK","1":"PBKS","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"15 May"},{"0":"RCB","1":"SRH","res":null,"venue":"Eden Gardens, Kolkata","date":"16 May"},{"0":"MI","1":"KKR","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"17 May"},{"0":"DC","1":"RR","res":null,"venue":"Eden Gardens, Kolkata","date":"18 May"},{"0":"CSK","1":"KKR","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"19 May"},{"0":"MI","1":"PBKS","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"20 May"},{"0":"SRH","1":"RR","res":null,"venue":"Eden Gardens, Kolkata","date":"21 May"},{"0":"RCB","1":"DC","res":null,"venue":"Eden Gardens, Kolkata","date":"22 May"},{"0":"KKR","1":"PBKS","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"23 May"},{"0":"RR","1":"RCB","res":null,"venue":"Eden Gardens, Kolkata","date":"25 May"},{"0":"CSK","1":"MI","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"26 May"},{"0":"DC","1":"SRH","res":null,"venue":"Eden Gardens, Kolkata","date":"28 May"},{"0":"KKR","1":"RR","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"30 May"}]`;

function get_num_finished_matches(data) {
    let num_finished = 0;
    for (let entry of data) {
    if( / won /.test(entry['res']) ) {
            num_finished += 1;
            continue;
        }
        break;
    }
    return num_finished;
}

function handleClick(data, extra_matches_to_compute) {
    console.log( "Data: ", data );

    const possibilities = wasm.get_chances(JSON.stringify(data), extra_matches_to_compute);

    console.log("Received from rust: ", possibilities);

    const scores = JSON.parse( possibilities );    // returns Javascript object

    const scores_arr = [];

    for (const team in scores) {
        if (Object.hasOwnProperty.call(scores, team)) {
            const percentage = scores[team];

            scores_arr.push({team, percentage});
        }
    }

    // sorted in descending order
    scores_arr.sort((a,b) => b.percentage - a.percentage);

    console.log(scores_arr);

    try{
        document.getElementById("score_table_rust").id; // this must cause a failure, if not available

        for (const score of scores_arr) {
            const tr = document.getElementById(`${score.team}_tr`);
            const t_chances = tr.children[1];

            t_chances.innerText = score.percentage;
        }

        document.getElementById("score_table_notice").innerText = `TILL ${extra_matches_to_compute + get_num_finished_matches(data)} Matches`;
    } catch {
        const table = document.createElement("table");
        table.id = "score_table_rust";
        const thead = document.createElement("thead");
        const tr = document.createElement("tr");
        const team_name = document.createElement("th");   // team name
        const team_chances = document.createElement("th");   // team chances
        team_name.innerText = "Team";
        team_chances.innerText = "Chances";
        tr.appendChild(team_name);
        tr.appendChild(team_chances);

        thead.appendChild(tr);
        table.appendChild(thead);

        const tbody = document.createElement("tbody");
        table.appendChild(tbody);

        for (const score of scores_arr) {
            const tr = document.createElement("tr");
            tr.id = `${score.team}_tr`;
            const t_name = document.createElement("td");
            const t_chances = document.createElement("td");

            t_name.innerText = score.team;
            t_chances.innerText = score.percentage;

            tr.appendChild(t_name);
            tr.appendChild(t_chances);

            tbody.appendChild(tr);
        }

        const caption = document.createElement("caption");
        const notice = document.createElement("strong");
        notice.id = "score_table_notice";
        notice.innerText = `TILL ${extra_matches_to_compute + get_num_finished_matches(data)} Matches`;
        const p = document.createElement("p");
        p.innerText = "Chances of teams qualifying for PlayOffs";

        caption.appendChild(notice)
        caption.appendChild(p);
        table.appendChild(caption);

        document.getElementById("chances_table").innerHTML = "";
        document.getElementById("chances_table").appendChild(table);
    }

}

async function fetchData() {
    return fetch("/.netlify/functions/league_matches")
            .then(res => {
                if(res.ok){
                    return res.json();
                }
                else throw Error("Couldn't fetch data");
            })
            .then(matches => {
                console.log("Got the data: ", matches.length);
                if(!Array.isArray(matches)) throw new Error("Received JSON is not an Array");

                return matches.map(match => ({  // we don't want the venue and date fields
                    '0': match['0'],
                    '1': match['1'],
                    res: match['res']
                }));
            })
}

fetchData()
.then(json_obj => json_obj)
.catch(err => {
    console.error("Fetch failed: ", err);
    console.log("Going with old data...");
    return JSON.parse( matches_str );
})
.then(data => {
    window.ipl_json_data = data;

    document.getElementById("extra_num").value = `${18 + get_num_finished_matches(window.ipl_json_data)}`;
})

document.querySelector("#num_matches_form").addEventListener("submit", (event) => {
    event.preventDefault();

    return new Promise((resolve, reject) => {
        let num_match = 30;

        try {
            num_match = parseInt(document.getElementById("extra_num").value)
        } catch {}

        resolve(handleClick(window.ipl_json_data, num_match - get_num_finished_matches(window.ipl_json_data)));
    }).then(() => {});
})
