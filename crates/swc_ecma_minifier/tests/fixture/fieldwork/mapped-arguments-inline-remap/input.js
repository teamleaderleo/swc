function mapped(b) {
    return (b = 2, arguments[0]);
}

console.log(mapped(1), mapped(1));
