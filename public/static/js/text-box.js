const inputs = document.querySelectorAll('.text-input');

for (input of inputs) {
    updateInput(input)
    input.addEventListener('change', (event) => {
        updateInput(input)
    });
}

function updateInput(input) {
    const classes = input.className.split(' ')
    if (input.value.length > 0) {
        if (!classes.includes("has-content")) {
            classes.push("has-content")
        }
    } else {
        const index = classes.indexOf("has-content");
        if (index > -1) {
            classes.splice(index, 1);
        }
    }
    input.className = classes.join(' ')
}