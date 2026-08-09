function callback(value, Constructor) {
    value instanceof Constructor;
}

function invalid(value) {
    value instanceof 2;
}

function directCallback(value, Constructor) {
    value instanceof Constructor;
}

function directInvalid(value) {
    value instanceof 2;
}

function control(value) {}
