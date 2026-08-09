function run(f) {
    f();
}

function sloppyOuterStrictChild(b) {
    run(function () {
        "use strict";
        b = 2;
    });
    return arguments[0];
}

function strictOuterSloppyChild(b) {
    "use strict";
    run(function () {
        b = 2;
    });
    return arguments[0];
}

console.log(sloppyOuterStrictChild(1), strictOuterSloppyChild(1));
