export function connectWiFi(): void {
    const ssidInput = document.getElementById("ssidInput") as HTMLInputElement;
    const passwordInput = document.getElementById(
        "passwordInput"
    ) as HTMLInputElement;

    const ssid = ssidInput.value.trim();
    const password = passwordInput.value.trim();

    const ssidError = document.getElementById(
        "ssidError"
    ) as HTMLParagraphElement;
    const passwordError = document.getElementById(
        "passwordError"
    ) as HTMLParagraphElement;

    // Clear previous errors
    ssidError.textContent = "";
    passwordError.textContent = "";

    let hasError = false;

    if (!ssid) {
        ssidError.textContent = "SSID cannot be empty.";
        hasError = true;
    }

    if (!password) {
        passwordError.textContent = "Password cannot be empty.";
        hasError = true;
    } else if (password.length < 8) {
        passwordError.textContent =
            "Password must be at least 8 characters long.";
        hasError = true;
    }

    if (hasError) {
        return;
    }

    fetch("/set_config", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ ssid, password }),
    })
        .then((response) => response.json())
        .then((data) => {
            console.log("Wi-Fi connection attempt response:", data);
        })
        .catch((error) => console.error("Error:", error));
}
