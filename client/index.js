import * as wasm from "ipl_pred"

function handleClick(data) {
    const possibilities = wasm.get_chances(JSON.stringify(data));


}

async function fetchData() {
    return fetch("/.netlify/functions/league_matches")
            .then(res => res.json())
            .then(matches => {
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
    .catch(console.error)
