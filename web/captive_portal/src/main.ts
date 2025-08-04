import "./style.css";
import { connectWiFi } from "./wifi";

const app = document.querySelector<HTMLDivElement>("#app");

if (app) {
    app.innerHTML = createHTMLContent();

    const connectWifiBtn = document.getElementById("connectWifiBtn");
    connectWifiBtn?.addEventListener("click", connectWiFi);

    // Event listeners to clear error messages on input
    document.getElementById("ssidInput")?.addEventListener("input", () => {
        document.getElementById("ssidError")!.textContent = "";
    });
    document.getElementById("passwordInput")?.addEventListener("input", () => {
        document.getElementById("passwordError")!.textContent = "";
    });
}

function createHTMLContent(): string {
    return `
        <div class="container">
            <h1>BTTF CLOCK</h1>

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
                <p id="ssidError" class="error-message"></p>
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
                <p id="passwordError" class="error-message"></p>
            </div>
            <div class="row">
                <button id="connectWifiBtn">Connect</button>
            </div>
        </div>

        <div class="tutorial-container">
            <h3>How to Configure</h3>
            <ul>
                <li><strong>Step 1:</strong> Enter your Wi-Fi credentials.</li>
                <li><strong>Step 2:</strong> Wait for the clock to restart.</li>
                <li><strong>Step 3:</strong> Access <strong>http://espclock.local</strong>.</li>
                <li><strong>Step 4:</strong> Configure your time zone.</li>
                <li><strong>Done!</strong></li>
            </ul>
        </div>
    `;
}
