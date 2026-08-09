function callback(value, Constructor) {
    value instanceof Constructor;
}

function invalid(value) {
    value instanceof 2;
}

function control(value) {}
