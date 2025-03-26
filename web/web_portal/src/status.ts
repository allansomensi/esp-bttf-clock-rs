export function fetchStatus(): void {
    fetch("/get_status", {
        method: "GET",
    })
        .then((response) => response.text())
        .then((data) => {
            const statusElement = document.getElementById(
                "status"
            ) as HTMLElement;
            statusElement.innerHTML = data;
        })
        .catch((error) => {
            console.error("Error fetching status:", error);
        });
}
