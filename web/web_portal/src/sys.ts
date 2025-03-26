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
