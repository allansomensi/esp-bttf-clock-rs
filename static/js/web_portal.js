document.addEventListener("DOMContentLoaded", function () {
    document.getElementById('digitsInput').value = '';
    document.getElementById('brightnessInput').value = '';
    document.getElementById('themeSelect').value = 'orange';
    document.getElementById('themeSelect').addEventListener('change', setTheme);

    populateTimezoneSelect();
});

function populateTimezoneSelect() {
    const timezones = [
        "UTC", "Africa/Abidjan", "Africa/Accra", "Africa/Addis_Ababa", "Africa/Algiers",
        "Africa/Asmara", "Africa/Bamako", "Africa/Bangui", "Africa/Banjul", "Africa/Bissau",
        "Africa/Blantyre", "Africa/Brazzaville", "Africa/Bujumbura", "Africa/Cairo",
        "Africa/Casablanca", "Africa/Ceuta", "Africa/Conakry", "Africa/Dakar",
        "Africa/Dar_es_Salaam", "Africa/Djibouti", "Africa/Douala", "Africa/El_Aaiun",
        "Africa/Freetown", "Africa/Gaborone", "Africa/Harare", "Africa/Johannesburg",
        "Africa/Juba", "Africa/Kampala", "Africa/Khartoum", "Africa/Kigali", "Africa/Kinshasa",
        "Africa/Lagos", "Africa/Libreville", "Africa/Lome", "Africa/Luanda",
        "Africa/Lusaka", "Africa/Malabo", "Africa/Maputo", "Africa/Maseru", "Africa/Mbabane",
        "Africa/Mogadishu", "Africa/Monrovia", "Africa/Nairobi", "Africa/Ndjamena",
        "Africa/Niamey", "Africa/Nouakchott", "Africa/Ouagadougou", "Africa/Porto-Novo",
        "Africa/Sao_Tome", "Africa/Tripoli", "Africa/Tunis", "Africa/Windhoek",
        "America/Adak", "America/Anchorage", "America/Araguaina", "America/Argentina/Buenos_Aires",
        "America/Argentina/Catamarca", "America/Argentina/Cordoba", "America/Argentina/Jujuy",
        "America/Argentina/La_Rioja", "America/Argentina/Mendoza", "America/Argentina/Rio_Gallegos",
        "America/Argentina/Salta", "America/Argentina/San_Juan", "America/Argentina/San_Luis",
        "America/Argentina/Tucuman", "America/Argentina/Ushuaia", "America/Asuncion",
        "America/Atikokan", "America/Bahia", "America/Bahia_Banderas", "America/Barbados",
        "America/Belem", "America/Belize", "America/Boa_Vista", "America/Bogota",
        "America/Boise", "America/Cambridge_Bay", "America/Campo_Grande", "America/Cancun",
        "America/Caracas", "America/Cayenne", "America/Chicago", "America/Chihuahua",
        "America/Costa_Rica", "America/Cuiaba", "America/Curacao", "America/Danmarkshavn",
        "America/Dawson", "America/Dawson_Creek", "America/Denver", "America/Detroit",
        "America/Edmonton", "America/Eirunepe", "America/El_Salvador", "America/Fort_Nelson",
        "America/Fortaleza", "America/Glace_Bay", "America/Godthab", "America/Goose_Bay",
        "America/Grand_Turk", "America/Guatemala", "America/Guayaquil", "America/Guyana",
        "America/Halifax", "America/Havana", "America/Hermosillo", "America/Indiana/Indianapolis",
        "America/Indiana/Knox", "America/Indiana/Marengo", "America/Indiana/Petersburg",
        "America/Indiana/Tell_City", "America/Indiana/Vevay", "America/Indiana/Vincennes",
        "America/Indiana/Winamac", "America/Inuvik", "America/Iqaluit", "America/Jamaica",
        "America/Juneau", "America/Kentucky/Louisville", "America/Kentucky/Monticello",
        "America/La_Paz", "America/Lima", "America/Los_Angeles", "America/Maceio",
        "America/Managua", "America/Manaus", "America/Martinique", "America/Matamoros",
        "America/Mazatlan", "America/Menominee", "America/Merida", "America/Metlakatla",
        "America/Mexico_City", "America/Miquelon", "America/Moncton", "America/Monterrey",
        "America/Montevideo", "America/New_York", "America/Nipigon", "America/Nome",
        "America/Noronha", "America/North_Dakota/Beulah", "America/North_Dakota/Center",
        "America/North_Dakota/New_Salem", "America/Ojinaga", "America/Panama",
        "America/Pangnirtung", "America/Paramaribo", "America/Phoenix", "America/Port-au-Prince",
        "America/Porto_Velho", "America/Puerto_Rico", "America/Rainy_River",
        "America/Recife", "America/Regina", "America/Resolute", "America/Rio_Branco",
        "America/Santarem", "America/Santiago", "America/Santo_Domingo", "America/Sao_Paulo",
        "America/Scoresbysund", "America/Sitka", "America/St_Johns", "America/Swift_Current",
        "America/Tegucigalpa", "America/Thule", "America/Tijuana", "America/Toronto",
        "America/Vancouver", "America/Whitehorse", "America/Winnipeg", "America/Yakutat",
        "Asia/Shanghai", "Asia/Tokyo", "Europe/Lisbon", "Europe/London", "Europe/Madrid",
        "Europe/Paris", "Europe/Rome", "Europe/Berlin"
    ];

    const timezoneSelect = document.getElementById('timezoneSelect');
    timezoneSelect.innerHTML = "";

    timezones.forEach(zone => {
        let option = document.createElement("option");
        option.value = zone;
        option.textContent = zone;
        timezoneSelect.appendChild(option);
    });
}

function setTimezone() {
    const timezone = document.getElementById('timezoneSelect').value;

    fetch('/set_timezone', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ timezone: timezone })
    })
        .then(response => response.text())
        .then(data => {
            document.getElementById('message').innerText = "Timezone updated to: " + timezone;
            fetchStatus();
        })
        .catch(error => console.error('Error updating timezone:', error));
}

function sendDigits() {
    let digits = document.getElementById('digitsInput').value;
    if (digits) {
        fetch('/set_digits?' + encodeURIComponent(digits), {
            method: 'GET'
        })
            .then(response => response.text())
            .then(data => {
                document.getElementById('message').innerText = "Digits sent: " + digits;
            })
            .catch(error => console.error('Error:', error));
    } else {
        alert("Please enter digits before sending.");
    }
}

function setBrightness() {
    let brightness = document.getElementById('brightnessInput').value;
    if (brightness >= 0 && brightness <= 7) {
        fetch('/set_brightness?' + brightness, {
            method: 'GET'
        })
            .then(response => response.text())
            .then(data => {
                document.getElementById('message').innerText = "Brightness set to " + brightness;
            })
            .catch(error => console.error('Error:', error));
    } else {
        alert("Brightness must be between 0 and 7.");
    }
}

function fetchStatus() {
    fetch('/get_status', {
        method: 'GET'
    })
        .then(response => response.text())
        .then(data => {
            document.getElementById('status').innerHTML = data;
        })
        .catch(error => console.error('Error fetching status:', error));
}

function syncTime() {
    fetch('/sync_time', {
        method: 'GET'
    })
        .then(response => response.text())
        .then(data => {
            document.getElementById('message').innerText = "Time synced successfully!";
            fetchStatus();
        })
        .catch(error => console.error('Error syncing time:', error));
}

function setTheme() {
    const theme = document.getElementById('themeSelect').value;

    fetch('/set_theme?' + encodeURIComponent(theme), {
        method: 'GET'
    })
        .then(response => response.text())
        .then(data => {
            document.getElementById('message').innerText = "Theme changed to: " + theme;

            if (theme === 'orange') {
                document.body.style.backgroundColor = '#ff7f0e';
                document.querySelector('.container').style.backgroundColor = '#fff5e6';
                document.querySelectorAll('button').forEach(btn => btn.style.backgroundColor = '#ff7f0e');
            } else if (theme === 'blue') {
                document.body.style.backgroundColor = '#007bff';
                document.querySelector('.container').style.backgroundColor = '#e6f2ff';
                document.querySelectorAll('button').forEach(btn => btn.style.backgroundColor = '#007bff');
            } else if (theme === 'green') {
                document.body.style.backgroundColor = '#28a745';
                document.querySelector('.container').style.backgroundColor = '#e6f9e0';
                document.querySelectorAll('button').forEach(btn => btn.style.backgroundColor = '#28a745');
            }
        })
        .catch(error => console.error('Error changing theme:', error));
}

function factoryReset() {
    if (confirm("Are you sure you want to reset to factory settings? This action cannot be undone.")) {
        fetch('/factory_reset', { method: 'GET' })
            .then(response => response.text())
            .then(data => {
                document.getElementById('message').innerText = "Factory reset initiated.";
            })
            .catch(error => console.error('Error initiating factory reset:', error));
    }
}

setInterval(fetchStatus, 30000);
fetchStatus();
