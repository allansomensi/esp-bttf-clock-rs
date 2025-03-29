import "./style.css";
import { connectWiFi } from "./wifi";

const app = document.querySelector<HTMLDivElement>("#app");

if (app) {
    app.innerHTML = createHTMLContent();

    const connectWifiBtn = document.getElementById("connectWifiBtn");
    connectWifiBtn?.addEventListener("click", connectWiFi);
}

function createHTMLContent(): string {
    return `
        <div class="container">
            <h1>Back to the Future Clock</h1>

            <h2>Wi-Fi Settings</h2>
            <div class="row">
                <input
                    type="text"
                    id="ssidInput"
                    placeholder="Enter SSID"
                    autocomplete="off"
                    autocapitalize="off"
                    spellcheck="false"
                />
            </div>
            <div class="row">
                <input
                    type="password"
                    id="passwordInput"
                    placeholder="Enter Password"
                    autocomplete="off"
                    minlength="8"
                    maxlength="40"
                    spellcheck="false"
                    required
                />
            </div>
            <div class="row">
                <button id="connectWifiBtn">Connect to Wi-Fi</button>
            </div>
        </div>
    `;
}
