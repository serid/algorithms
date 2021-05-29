// Church enconding for numbers, booleans, lists + Z combinator and recursion demos

function* range(a, b) {
    if (b === undefined) {
        b = a
        a = 0
    }
    for (let i = a; i < b; i++) {
        yield i;
    }
}

function mapPair(p, f, g) {
    return [f(p[0]), g(p[1])]
}

let zero = s => z => z;

// Numbers
let Succ = n => s => z => s(n(s)(z));

let Add = n => n(Succ);
let Add2 = n => m => n(Succ)(m);
let Add3 = n => m => s => z => n(s)(m(s)(z));

let Mul = n => m => s => n(m(s));
let Mul2 = n => m => s => z => n(m(s))(z);
let Mul3 = n => m => n(Add(m))(zero);

// Booleans
let True = a => b => a;
let False = a => b => b;

let And = a => b => a(b)(False);

let Or = a => a(True);
let Or2 = a => b => a(True)(b);

let Not = a => a(False)(True);

let IsZero = n => n(True(False))(True);
let IsZero2 = n => n(False)(Not)(False);

// Pairs
let ConsP = x => y => (f => f(x)(y));
let First = p => p(True);
let Second = p => p(False);

// Predecessor
let Φ = p => ConsP(Succ(First(p)))(First(p));
let Pred = n => Second(n(Φ)(ConsP(zero)(zero)));
let Subtract = n => m => m(Pred)(n);

// Less than or Equal
let Le = n => m => IsZero(Subtract(n)(m));

let Eq = n => m => And(Le(n)(m))(Le(m)(n));

// Recursion
let Y = r => (x => r(x(x)))(x => r(x(x)));
let Z = r => (x => r(v => x(x)(v)))(x => r(v => x(x)(v)));

// Helper function for lazy evaluation
let LazyIf = c => a => b => c(a)(b)(null)

/*
// List (right fold operation)
// a is accumulator
// f is combinator
// e is element
// t is tail
let Nil = (c => a => a);
let Cons = e => t => (c => a => c(e)(t(c)(a)));
let IsNil = l => l(e => a => Or(True)(a))(False);

function ListToCList(arr) {
    let ls = Nil;
    for (const e of arr) {
        ls = Cons(e)(ls);
    }
    return ls;
}

function CListToList(ls) {
    return ls(e => a => [e].concat(a))([]);
}
*/

// List (left fold operation)
// a is accumulator
// f is combinator
// e is element
// t is tail
let Nil = (c => a => a);
let Cons = e => t => (c => a => t(c)(c(a)(e)));
let IsNil = l => l(a => e => Or(True)(a))(False);
let Head = l => defaut_ => l(a => e => First(a)(a)(ConsP(True)(e)))(ConsP(False)(defaut_));
let LMap = f => l => l(a => e => Cons(f(e))(a))(Nil);
let Count = l => l(a => e => Succ(a))(zero);

function NumberToCNumber(n) {
    let cnumber = zero;

    for (const i of range(n)) {
        cnumber = Succ(cnumber);
    }

    return cnumber;
}

function CNumberToNumber(n) {
    return n(x => x + 1)(0);
}

function BoolToCBool(b) {
    return b ? True : False;
}

function CBoolToBool(b) {
    return b(true)(false);
}

function PairToCPair(p) {
    return ConsP(p[0])(p[1]);
}

function CPairToPair(p) {
    return p(x => y => [x, y]);
}

function ListToCList(arr) {
    return arr.reduceRight((a, e) => Cons(e)(a), Nil);
}

function CListToList(ls) {
    return ls(a => e => a.concat([e]))([]);
}

function main() {
    let zero = s => z => z;
    let one = s => z => s(z);
    let two = s => z => s(s(z));
    let three = s => z => s(s(s(z)));

    let five = Add(two)(three);
    let ten = Mul(two)(five);
    let thirty = Mul(three)(ten);

    console.log("10: " + CNumberToNumber(ten));

    console.log("IsZero(0): " + CBoolToBool(IsZero(zero)));
    console.log("IsZero(10): " + CBoolToBool(IsZero(ten)));

    console.log("10 <= 30: " + CBoolToBool(Le(ten)(thirty)));
    console.log("30 <= 10: " + CBoolToBool(Le(thirty)(ten)));

    console.log("2 + 2 == 4: " + CBoolToBool(Eq(Add(two)(two))(NumberToCNumber(4))));

    console.log("sum: " + CNumberToNumber(Z(r => n => LazyIf(IsZero(n))(_ => zero)(_ => n(Succ)(r(Pred(n)))))(five)));

    let fact1 = f => n => LazyIf(IsZero(n))(_ => one)(_ => Mul(n)(f(Pred(n))));
    let fact = Z(fact1);

    console.log("fact(6): " + CNumberToNumber(fact(NumberToCNumber(6))));

    let ls = LMap(NumberToCNumber)(ListToCList([1, 2, 3]));
    console.log("list: " + CListToList(LMap(CNumberToNumber)(ls)));
    console.log("head: " + mapPair(CPairToPair(Head(ls)(zero)), CBoolToBool, CNumberToNumber));
    console.log("count: " + CNumberToNumber(Count(ls)));
}

main();