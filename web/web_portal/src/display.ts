export function setDisplayBrightness(): void {
    const brightnessInput = document.getElementById(
        "brightnessInput"
    ) as HTMLInputElement;
    const highPowerSwitch = document.getElementById(
        "highPowerSwitch"
    ) as HTMLInputElement;
    const brightness = Number(brightnessInput.value);

    if (brightness < 0 || brightness > 7) {
        alert("Brightness must be between 0 and 7.");
        return;
    }

    if (!highPowerSwitch.checked && brightness > 3) {
        alert(
            "Brightness is limited to 3. Enable 'High Power Mode' for maximum brightness."
        );
        return;
    }

    fetch(`/set_brightness?${brightness}`, {
        method: "GET",
    })
        .then((response) => response.text())
        .then(() => {
            const messageElement = document.getElementById(
                "message"
            ) as HTMLElement;
            messageElement.innerText = "Brightness set to " + brightness;
        })
        .catch((error) => {
            console.error("Error:", error);
        });
}
