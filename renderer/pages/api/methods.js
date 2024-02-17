const url = "http://localhost:11424/check"
const requestOption = (method) => {
    return {
        method: method,
        headers: {
            'Content-Type': 'application/json'
        }
    }
}

export async function checkHardware() {
    // fetch the url
    try {
        const response = await fetch(url, requestOption('GET'))
        const data = await response.json()
        // check content of the response
        return data
    } catch (e) {
        console.error(e)
        return {"error": "An error occurred"}
    }
}
    