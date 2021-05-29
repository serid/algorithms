// Implementation of a lambda calculus interpreter using de bruijn indices as described in
// https://www.cs.cornell.edu/courses/cs4110/2012fa/lectures/lecture14.pdf

trait BoxMapExt<T> {
    fn map(self, _: impl Fn(T) -> T) -> Self;
}

// Map a Box in place
impl<T> BoxMapExt<T> for Box<T> {
    fn map(mut self, f: impl Fn(T) -> T) -> Self {
        *self = f(*self);
        self
    }
}

enum UnboxedOrBoxed<T> {
    Unboxed(T),
    Boxed(Box<T>),
}

#[inline]
fn box_maybe_bind<T>(mut box_: Box<T>, f: impl FnOnce(T) -> UnboxedOrBoxed<T>) -> Box<T> {
    let res = f(*box_);
    match res {
        UnboxedOrBoxed::Unboxed(v) => {
            *box_ = v;
            box_
        }
        UnboxedOrBoxed::Boxed(new_box) => new_box
    }
}

#[inline]
fn box_map<T>(mut box_: Box<T>, f: impl FnOnce(T) -> T) -> Box<T> {
    *box_ = f(*box_);
    box_
}


type Index = usize;

#[derive(Debug, Clone)]
enum Expr {
    Var(Index),
    Lambda(Box<Expr>),
    Application(Box<Expr>, Box<Expr>),
}

use Expr::*;

fn var(n: Index) -> Expr {
    Expr::Var(n)
}

fn lambda(e: Expr) -> Expr {
    Expr::Lambda(Box::new(e))
}

fn application(e1: Expr, e2: Expr) -> Expr {
    Expr::Application(Box::new(e1), Box::new(e2))
}


// i may be negative
fn shift(i: isize, c: Index, e: Box<Expr>) -> Box<Expr> {
    box_map(e, |e| match e {
        Var(n) => if n < c { e } else { Var(Result::unwrap(usize::try_from(Result::unwrap(isize::try_from(n)) + i))) },
        Lambda(e1) => Lambda(shift(i, c + 1, e1)),
        Application(e1, e2) => Application(shift(i, c, e1), shift(i, c, e2)),
    })
}

fn replace(e: Box<Expr>, fillin: Box<Expr>, m: Index) -> Box<Expr> {
    box_maybe_bind(e, |e| match e {
        Var(n) => if n == m { UnboxedOrBoxed::Boxed(fillin) } else { UnboxedOrBoxed::Unboxed(e) },
        Lambda(e1) => UnboxedOrBoxed::Unboxed(Lambda(replace(e1, shift(1, 0, fillin), m + 1))),
        Application(e1, e2) => {
            let clone1 = fillin.clone();
            let clone2 = fillin;
            UnboxedOrBoxed::Unboxed(Application(replace(e1, clone1, m), replace(e2, clone2, m)))
        }
    })
}

fn replace2(e: Box<Expr>, fillin: &Expr, m: Index) -> Box<Expr> {
    box_map(e, |e| match e {
        Var(n) => if n == m { fillin.clone() } else { e },
        Lambda(e1) => Lambda(replace2(e1, &shift(1, 0, Box::new(fillin.clone())), m + 1)),
        Application(e1, e2) => Application(replace2(e1, fillin, m), replace2(e2, fillin, m)),
    })
}

fn reduce(e: Expr) -> Option<Box<Expr>> {
    match e {
        Application(e1, e2) =>
            match *e1 {
                Lambda(e1) => Some(shift(-1, 0, replace(e1, shift(1, 0, e2), 0))),
                _ => None,
            },
        _ => None,
    }
}

pub fn run() {
    let e = application(lambda(lambda(application(var(1), var(2)))), var(1));
    //
    // println!("{:?} {:?}", replace(Box::new(var(1)), Box::new(var(3)), 1),
    //          replace(Box::new(var(2)), Box::new(var(3)), 1));
    //
    // println!("{:?}", shift(-1, 0, Box::new(lambda(application(var(3), var(2))))));


    // let e = application(lambda(var(0)), var(10));

    let res = reduce(e);

    println!("{:?}", res)
}