const effects = [];

({
    valueOf() {
        effects.push("plus");
        return 1;
    },
}) + 1;

({
    valueOf() {
        effects.push("eq");
        return 1;
    },
}) == 1;

let mixedBigIntThrew = false;
try {
    1n + 1;
} catch {
    mixedBigIntThrew = true;
}

({}) === 1;

console.log(effects.join(","), mixedBigIntThrew);
