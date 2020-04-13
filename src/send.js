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

function split_name(source) {
	var split = source.split("::");
	var variable = split.pop();
	var module = split.join("::");

	return {
		variable: variable,
		module: module
	}
}

function changed_value(source, value, data_type) {
	let names = split_name(source);

	// Create a line and add it to the map
	var line;
	if (data_type == "string") {
		line = "const " + names.variable + ": &str = \"" + value + "\";"
	} else {
		line = "const " + names.variable + ": " + data_type + " = " + value + ";"
	}
	changed_values.set(source, line);

	// Print the whole map for this source
	let output = "";
	for (let [map_source, line] of changed_values) {
		if (names.module == split_name(map_source).module) {
			output += line + "\n";
		}
	}
	var output_text = document.getElementById(names.module.replace("::", "_") + "_output");
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
