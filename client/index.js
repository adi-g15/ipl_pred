import * as wasm from "ipl_pred"

function handleClick(data) {

}

async function fetchData() {
    return fetch("/")
}

handleClick( await fetchData() );
