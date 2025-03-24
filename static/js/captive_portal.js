function connectWiFi() {
    let ssid = document.getElementById("ssidInput").value;
    let password = document.getElementById("passwordInput").value;

    if (ssid && password) {
        fetch("/set_config", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ ssid: ssid, password: password }),
        })
            .then((response) => response.json())
            .then((data) => {
                document.getElementById("message").innerText = data.message;
            })
            .catch((error) => console.error("Error:", error));
    } else {
        alert("Please fill in both SSID and password.");
    }
}
