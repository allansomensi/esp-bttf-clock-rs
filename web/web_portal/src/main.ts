import "./style.css";
import { setDisplayBrightness } from "./display";
import { populateTimezoneSelect, setTimezone, syncTime } from "./time";
import { factoryReset } from "./sys";
import { setTheme } from "./theme";
import { fetchStatus } from "./status";

const app = document.querySelector<HTMLDivElement>("#app");

if (app) {
    app.innerHTML = createHTMLContent();

    const setDisplayBrightnessBtn = document.getElementById(
        "setDisplayBrightnessBtn"
    );
    setDisplayBrightnessBtn?.addEventListener("click", setDisplayBrightness);

    const syncTimeBtn = document.getElementById("syncTimeBtn");
    syncTimeBtn?.addEventListener("click", syncTime);

    const setTimezoneBtn = document.getElementById("setTimezoneBtn");
    setTimezoneBtn?.addEventListener("click", setTimezone);

    const factoryResetBtn = document.getElementById("factoryResetBtn");
    factoryResetBtn?.addEventListener("click", factoryReset);

    const brightnessInput = document.getElementById(
        "brightnessInput"
    ) as HTMLInputElement;
    const themeSelect = document.getElementById(
        "themeSelect"
    ) as HTMLSelectElement;

    brightnessInput.value = "";
    themeSelect.value = "orange";

    themeSelect.addEventListener("change", setTheme);

    populateTimezoneSelect();

    setInterval(fetchStatus, 30000);
    fetchStatus();
}

function createHTMLContent(): string {
    return `
        <div class="container">
            <h1>ESP Server</h1>
            <p id="message" class="message"></p>

            <!-- Display -->
            <h2>Display</h2>
            <div class="row">
                <input
                    type="number"
                    id="brightnessInput"
                    placeholder="Brightness (0-7)"
                    min="0"
                    max="7"
                    autocomplete="off"
                />
                <button id="setDisplayBrightnessBtn">Set Brightness</button>
            </div>

            <!-- Sync Time Button -->
            <h2>Sync Time</h2>
            <div class="row">
                <button id="syncTimeBtn">Sync Time with SNTP</button>
            </div>

            <!-- Theme Selector -->
            <h2>Theme</h2>
            <div class="row">
                <select id="themeSelect">
                    <option value="orange">Orange</option>
                    <option value="blue">Blue</option>
                    <option value="green">Green</option>
                </select>
            </div>

            <!-- Status Section -->
            <h2>Status</h2>
            <div id="status">
                <p><strong>SSID:</strong> <span id="ssid">Loading...</span></p>
                <p>
                    <strong>Timezone:</strong>
                    <span id="timezone">Loading...</span>
                </p>
                <p><strong>Time:</strong> <span id="time">Loading...</span></p>
            </div>

            <!-- Timezone Selector -->
            <h2>Timezone</h2>
            <div class="row">
                <select id="timezoneSelect"></select>
                <button id="setTimezoneBtn">Set Timezone</button>
            </div>

            <!-- Factory Reset Button -->
            <h2>Factory Reset</h2>
            <div class="row">
                <button id="factoryResetBtn">Restore Factory Settings</button>
            </div>
        </div>
    `;
}
