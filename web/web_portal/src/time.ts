import { fetchStatus } from "./status";
import { timezones } from "./timezones";

export function syncTime(): void {
    fetch("/sync_time", {
        method: "GET",
    })
        .then((response: Response) => response.text())
        .then(() => {
            const messageElement = document.getElementById("message");
            if (messageElement) {
                messageElement.innerText = "Time synced successfully!";
            }
            fetchStatus();
        })
        .catch((error: Error) => console.error("Error syncing time:", error));
}

export function setTimezone(): void {
    const timezoneSelect = document.getElementById(
        "timezoneSelect"
    ) as HTMLSelectElement;
    const timezone = timezoneSelect.value;

    fetch("/set_timezone", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ timezone: timezone }),
    })
        .then((response) => response.text())
        .then(() => {
            const messageElement = document.getElementById(
                "message"
            ) as HTMLElement;
            messageElement.innerText = "Timezone updated to: " + timezone;
            fetchStatus();
        })
        .catch((error) => {
            console.error("Error updating timezone:", error);
        });
}

export function populateTimezoneSelect(): void {
    const timezoneSelect = document.getElementById(
        "timezoneSelect"
    ) as HTMLSelectElement;
    timezoneSelect.innerHTML = "";

    timezones.forEach((tz) => {
        let option = document.createElement("option");
        option.value = tz;
        option.textContent = tz;
        timezoneSelect.appendChild(option);
    });
}
