export function setDisplayBrightness(): void {
    const brightnessInput = document.getElementById(
        "brightnessInput"
    ) as HTMLInputElement;
    const brightness = Number(brightnessInput.value);

    if (brightness >= 0 && brightness <= 7) {
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
    } else {
        alert("Brightness must be between 0 and 7.");
    }
}
