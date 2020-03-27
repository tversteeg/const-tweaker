function send(source, value, data_type) {
	// Change the label
	var label_element = document.getElementById(source + '_label');
	if (label_element) {
		label_element.innerHTML = value;
	}

	// Make the request
	fetch('/set/' + data_type, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({key: source, value: value})
	}).catch(err => {
		document.getElementById('status').textContent = 'HTTP Error: ' + err;
	});
}

function poll() {
	// Make the request
	fetch('/should_refresh')
		.then(response => response.text())
		.then(body => {
			if (body == "refresh") {
				console.log("Reloading page");
				location.reload();
			}
		})
		.catch(err => {
			document.getElementById('status').textContent = 'HTTP Error: ' + err;
		});
}

// Poll every second
setInterval(poll, 1000);
