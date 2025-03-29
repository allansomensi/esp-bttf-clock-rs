export function setTheme(): void {
    const theme = (document.getElementById("themeSelect") as HTMLSelectElement)
        .value;

    fetch("/set_theme?" + encodeURIComponent(theme), {
        method: "GET",
    })
        .then((response) => response.text())
        .catch((error) => console.error("Error changing theme:", error));
}
