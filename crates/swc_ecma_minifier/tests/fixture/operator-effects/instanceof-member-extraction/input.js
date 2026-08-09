function callback(value, Constructor) {
    [value instanceof Constructor][1];
}

function invalid(value) {
    [value instanceof 2][1];
}

function directCallback(value, Constructor) {
    value instanceof Constructor;
}

function directInvalid(value) {
    value instanceof 2;
}

function directCallbackNotLast(value, Constructor) {
    value instanceof Constructor;
    return value;
}

function directInvalidNotLast(value) {
    value instanceof 2;
    return value;
}

function control(value) {
    [value === 2][1];
}
