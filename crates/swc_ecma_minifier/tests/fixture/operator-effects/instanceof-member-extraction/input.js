function callback(value, Constructor) {
    [value instanceof Constructor][1];
}

function invalid(value) {
    [value instanceof 2][1];
}

function control(value) {
    [value === 2][1];
}
