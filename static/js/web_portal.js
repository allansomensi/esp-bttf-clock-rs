document.addEventListener("DOMContentLoaded", function () {
    document.getElementById('digitsInput').value = '';
    document.getElementById('brightnessInput').value = '';
    document.getElementById('themeSelect').value = 'orange';
    document.getElementById('themeSelect').addEventListener('change', setTheme);
});

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
