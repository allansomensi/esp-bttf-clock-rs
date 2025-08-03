export function setTheme(): void {
    const theme = (document.getElementById("themeSelect") as HTMLSelectElement)
        .value;

    fetch("/set_theme?" + encodeURIComponent(theme), {
        method: "GET",
    })
        .then((response) => response.text())
        .then(() => {
            const messageElement = document.getElementById(
                "message"
            ) as HTMLElement;
            messageElement.innerText = "Theme changed!";
        })
        .catch((error) => console.error("Error changing theme:", error));
}
