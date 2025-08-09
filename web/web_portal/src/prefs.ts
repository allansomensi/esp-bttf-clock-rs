export function setHourFormat(): void {
    const hourFormatSwitch = document.getElementById(
        "hourFormatSwitch"
    ) as HTMLInputElement;
    const messageElement = document.getElementById("message") as HTMLElement;

    const value = hourFormatSwitch.checked ? 1 : 0;
    const formatText = hourFormatSwitch.checked ? "24h" : "12h";

    fetch(`/set_hour_format?${value}`, {
        method: "GET",
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to set hour format.');
            }
        })
        .then(() => {
            messageElement.innerText = `Hour format set to ${formatText}`;
            messageElement.className = "message success";
        })
        .catch(error => {
            console.error("Error:", error);
            messageElement.innerText = "Error: Could not set hour format.";
            messageElement.className = "message error";
        });
}
