import "./style.css";
import { setDisplayBrightness } from "./display";
import { populateTimezoneSelect, setTimezone, syncTime } from "./time";
import { factoryReset } from "./sys";
import { setTheme } from "./theme";
import { setHourFormat } from "./prefs";
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

    const hourFormatSwitch = document.getElementById(
        "hourFormatSwitch"
    ) as HTMLInputElement;

    brightnessInput.value = "";
    themeSelect.value = "original";

    themeSelect.addEventListener("change", setTheme);
    hourFormatSwitch.addEventListener("change", setHourFormat);

    populateTimezoneSelect();

    setInterval(fetchStatus, 30000);
    fetchStatus();
}

function createHTMLContent(): string {
    return `
        <div class="container">
            <h1>ESP-BTTF-CLOCK-RS</h1>
            <p id="message" class="message"></p>

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
            <div class="row setting-row">
                <span>Hour Format</span>
                <div class="switch-container">
                    <span class="switch-label">12h</span>
                    <label class="switch">
                        <input type="checkbox" id="hourFormatSwitch">
                        <span class="slider"></span>
                    </label>
                    <span class="switch-label">24h</span>
                </div>
            </div>

            <h2>Sync Time</h2>
            <div class="row">
                <button id="syncTimeBtn">Sync Time with SNTP</button>
            </div>

            <h2>Theme</h2>
            <div class="row">
                <select id="themeSelect">
                    <option value="original">Original</option>
                    <option value="hoverboard">Hoverboard</option>
                    <option value="plutonium">Plutonium</option>
                    <option value="oldwest">Old West</option>
                    <option value="cafe80s">Cafe 80's</option>
                </select>
            </div>

            <h2>Status</h2>
            <div id="status">
                <p><strong>SSID:</strong> <span id="ssid">Loading...</span></p>
                <p>
                    <strong>Timezone:</strong>
                    <span id="timezone">Loading...</span>
                </p>
                <p><strong>Time:</strong> <span id="time">Loading...</span></p>
            </div>

            <h2>Timezone</h2>
            <div class="row">
                <select id="timezoneSelect"></select>
                <button id="setTimezoneBtn">Set Timezone</button>
            </div>

            <h2>Factory Reset</h2>
            <div class="row">
                <button id="factoryResetBtn">Restore Factory Settings</button>
            </div>
        </div>
    `;
}
