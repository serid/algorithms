function* range(a, b) {
    if (b === undefined) {
        b = a
        a = 0
    }
    for (let i = a; i < b; i++) {
        yield i;
    }
    return "doge"
}

function* gen() {
    yield 10;
    yield 20;
    return 53;
}

function main() {
    let i = gen();

    for (const iter of range(0, 10)) {
        console.log(i.next());
    }
}

main()