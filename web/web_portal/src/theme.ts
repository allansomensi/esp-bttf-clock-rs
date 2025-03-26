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
            messageElement.innerText = "Theme changed to: " + theme;

            const container = document.querySelector(
                ".container"
            ) as HTMLElement;
            const buttons = document.querySelectorAll(
                "button"
            ) as NodeListOf<HTMLButtonElement>;

            if (theme === "orange") {
                document.body.style.backgroundColor = "#ff7f0e";
                container.style.backgroundColor = "#fff5e6";
                buttons.forEach(
                    (btn) => (btn.style.backgroundColor = "#ff7f0e")
                );
            } else if (theme === "blue") {
                document.body.style.backgroundColor = "#007bff";
                container.style.backgroundColor = "#e6f2ff";
                buttons.forEach(
                    (btn) => (btn.style.backgroundColor = "#007bff")
                );
            } else if (theme === "green") {
                document.body.style.backgroundColor = "#28a745";
                container.style.backgroundColor = "#e6f9e0";
                buttons.forEach(
                    (btn) => (btn.style.backgroundColor = "#28a745")
                );
            }
        })
        .catch((error) => console.error("Error changing theme:", error));
}
