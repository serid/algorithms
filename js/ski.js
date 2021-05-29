// Church enconding for numbers, booleans, lists + Z combinator and recursion demos

function ski() {
    let i = x => x(a => b => c => a(c)(b(c)))(a => b => a);
    let identity = i(i);
    let k = i(i(identity));
    let s = i(k);

    let zero = k(identity);

    let True = k;
    let False = k(identity);
}

function main() {
    console.log("10: " + CNumberToNumber(ten));
}

main();