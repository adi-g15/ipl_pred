import * as wasm from "../pkg";

const matches_str = `[{"0":"MI","1":"RCB","res":"RCB won by 2 wickets","venue":null,"date":"9 Apr"},{"0":"CSK","1":"DC","res":"DC won by 7 wickets","venue":null,"date":"10 Apr"},{"0":"SRH","1":"KKR","res":"KKR won by 10 runs","venue":null,"date":"11 Apr"},{"0":"RR","1":"PBKS","res":"PBKS won by 4 runs","venue":null,"date":"12 Apr"},{"0":"KKR","1":"MI","res":"MI won by 10 runs","venue":null,"date":"13 Apr"},{"0":"SRH","1":"RCB","res":"RCB won by 6 runs","venue":null,"date":"14 Apr"},{"0":"RR","1":"DC","res":"RR won by 3 wickets","venue":null,"date":"15 Apr"},{"0":"PBKS","1":"CSK","res":"CSK won by 6 wickets","venue":null,"date":"16 Apr"},{"0":"MI","1":"SRH","res":"MI won by 13 runs","venue":null,"date":"17 Apr"},{"0":"RCB","1":"KKR","res":"RCB won by 38 runs","venue":null,"date":"18 Apr"},{"0":"DC","1":"PBKS","res":"DC won by 6 wickets","venue":null,"date":"19 Apr"},{"0":"CSK","1":"RR","res":null,"venue":"Wankhede Stadium, Mumbai","date":"20 Apr"},{"0":"DC","1":"MI","res":null,"venue":"M. A. Chidambaram Stadium, Chennai","date":"21 Apr"},{"0":"PBKS","1":"SRH","res":null,"venue":"M. A. Chidambaram Stadium, Chennai","date":"22 Apr"},{"0":"KKR","1":"CSK","res":null,"venue":"Wankhede Stadium, Mumbai","date":"23 Apr"},{"0":"RCB","1":"RR","res":null,"venue":"Wankhede Stadium, Mumbai","date":"24 Apr"},{"0":"PBKS","1":"MI","res":null,"venue":"M. A. Chidambaram Stadium, Chennai","date":"25 Apr"},{"0":"RR","1":"KKR","res":null,"venue":"Wankhede Stadium, Mumbai","date":"26 Apr"},{"0":"CSK","1":"RCB","res":null,"venue":"Wankhede Stadium, Mumbai","date":"27 Apr"},{"0":"SRH","1":"DC","res":null,"venue":"M. A. Chidambaram Stadium, Chennai","date":"28 Apr"},{"0":"PBKS","1":"KKR","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"29 Apr"},{"0":"DC","1":"RCB","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"30 Apr"},{"0":"CSK","1":"SRH","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"1 May"},{"0":"MI","1":"RR","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"2 May"},{"0":"DC","1":"KKR","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"3 May"},{"0":"PBKS","1":"RCB","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"4 May"},{"0":"MI","1":"CSK","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"5 May"},{"0":"RR","1":"SRH","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"6 May"},{"0":"PBKS","1":"DC","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"7 May"},{"0":"KKR","1":"RCB","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"8 May"},{"0":"SRH","1":"MI","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"9 May"},{"0":"RR","1":"CSK","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"10 May"},{"0":"RCB","1":"PBKS","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"11 May"},{"0":"SRH","1":"CSK","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"12 May"},{"0":"KKR","1":"DC","res":null,"venue":"Narendra Modi Stadium, Ahmedabad","date":"13 May"},{"0":"RR","1":"MI","res":null,"venue":"Arun Jaitley Stadium, Delhi","date":"14 May"},{"0":"CSK","1":"PBKS","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"15 May"},{"0":"RCB","1":"SRH","res":null,"venue":"Eden Gardens, Kolkata","date":"16 May"},{"0":"MI","1":"KKR","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"17 May"},{"0":"DC","1":"RR","res":null,"venue":"Eden Gardens, Kolkata","date":"18 May"},{"0":"CSK","1":"KKR","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"19 May"},{"0":"MI","1":"PBKS","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"20 May"},{"0":"SRH","1":"RR","res":null,"venue":"Eden Gardens, Kolkata","date":"21 May"},{"0":"RCB","1":"DC","res":null,"venue":"Eden Gardens, Kolkata","date":"22 May"},{"0":"KKR","1":"PBKS","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"23 May"},{"0":"RR","1":"RCB","res":null,"venue":"Eden Gardens, Kolkata","date":"25 May"},{"0":"CSK","1":"MI","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"26 May"},{"0":"DC","1":"SRH","res":null,"venue":"Eden Gardens, Kolkata","date":"28 May"},{"0":"KKR","1":"RR","res":null,"venue":"M. Chinnaswamy Stadium, Bengaluru","date":"30 May"}]`;

function handleClick(data) {
    console.log( "Data: ", data );
    const extra_matches_to_compute = 20;

    const possibilities = wasm.get_chances(matches_str, extra_matches_to_compute);

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
    .then(json_obj => handleClick(json_obj))
    .catch(err => {
        console.error("Fetch failed: ", err);
        console.log("Going with old data...");
        handleClick(JSON.parse( matches_str ));
    })
