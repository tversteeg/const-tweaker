var changed_values = new Map();

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

	// Append it to the output text
	changed_value(source, value, data_type)
}

function changed_value(source, value, data_type) {
	var split = source.split("::");
	var module = split[0];
	var variable = split[1];

	// Create a line and add it to the map
	var line;
	if (data_type == "string") {
		line = "const " + variable + ": &str = \"" + value + "\";"
	} else {
		line = "const " + variable + ": " + data_type + " = " + value + ";"
	}
	changed_values.set(source, line);

	// Print the whole map for this source
	let output = "";
	for (let [map_source, line] of changed_values) {
		if (module == source.split("::")[0]) {
			output += line + "\n";
		}
	}
	var output_text = document.getElementById(module + "_output");
	output_text.value = output;
}

function copy_text(source) {
	// Select the text area
	var output_text = document.getElementById(source + "_output");
	output_text.select();
	output_text.setSelectionRange(0, 99999); /*For mobile devices*/

	// Copy it
	document.execCommand("copy");
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

// Poll every 3 seconds
setInterval(poll, 3000);
