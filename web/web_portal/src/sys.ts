export function factoryReset(): void {
    if (
        confirm(
            "Are you sure you want to reset to factory settings? This action cannot be undone."
        )
    ) {
        fetch("/factory_reset", { method: "GET" })
            .then((response) => response.text())
            .then(() => {
                const messageElement = document.getElementById(
                    "message"
                ) as HTMLElement;
                messageElement.innerText = "Factory reset initiated.";
            })
            .catch((error) => {
                console.error("Error initiating factory reset:", error);
            });
    }
}

export function handlePowerModeChange(): void {
    const highPowerSwitch = document.getElementById(
        "highPowerSwitch"
    ) as HTMLInputElement;

    if (highPowerSwitch.checked) {
        if (
            !confirm(
                "Warning: To enable High Power Mode, a 5V power supply that provides at least 1.5A is recommended. Standard computer USB ports (which usually provide 0.5A) may not be sufficient, causing instability in the device. Do you wish to continue?"
            )
        ) {
            highPowerSwitch.checked = false;
        }
    }
}
