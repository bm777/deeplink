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
        return await response.json()
    } catch (e) {
        console.error(e)
        return {"error": "An error occurred"}
    }
}
export async function getIpAddress() {
    try {
        const response = await fetch("https://api.ipify.org?format=json", requestOption('GET'))
        return await response.json()
    } catch (e) {
        console.error(e)
        return {"error": "An error occurred"}
    }
}
    