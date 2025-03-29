export function connectWiFi(): void {
    const ssid = (document.getElementById("ssidInput") as HTMLInputElement)
        .value;
    const password = (
        document.getElementById("passwordInput") as HTMLInputElement
    ).value;
    const messageElement = document.getElementById("message");

    if (ssid && password) {
        fetch("/set_config", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ ssid, password }),
        })
            .then((response) => response.json())
            .then((data: { message: string }) => {
                if (messageElement) {
                    messageElement.innerText = data.message;
                }
            })
            .catch((error) => console.error("Error:", error));
    } else {
        alert("Please fill in both SSID and password.");
    }
}
