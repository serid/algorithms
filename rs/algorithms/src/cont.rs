enum Void {}

fn id<T>(x: T) -> T {
    x
}

fn result_unwrap_no_debug<T, E>(x: Result<T, E>, message: &str) -> T {
    x.unwrap_or_else(|_| panic!("{}", message))
}

struct OnceFnRef<'a, T, R>(&'a mut Option<Box<dyn FnOnce(T) -> R + 'a>>);

impl<'a, T, R> OnceFnRef<'a, T, R> {
    fn make_option_box(f: impl FnOnce(T) -> R + 'static) -> Option<Box<dyn FnOnce(T) -> R>> {
        Some(Box::new(f))
    }

    fn call(self, value: T) -> R {
        result_unwrap_no_debug(self.call_fallible(value), "error")
    }

    fn call_fallible(self, value: T) -> Result<R, T> {
        match self.0.take() {
            Some(f) => Ok(f(value)),
            None => Err(value),
        }
    }
}

// Second Box can be replaced with & I think
struct Cont<R, T>(Box<dyn for<'a> FnMut(OnceFnRef<'a, T, R>) -> R>);

impl<R, T: 'static + Clone> Cont<R, T> {
    fn pure(value: T) -> Cont<R, T> {
        Cont(Box::new(move |f| f.call(value.clone())))
    }
}

impl<R: 'static, T: 'static> Cont<R, T> {
    // fn bind<U>(&mut self, k: impl FnOnce(T) -> Cont<R, U>) -> Cont<R, U> {
    //     self.run(Box::new(|value| k(value)))
    // }

    fn map<U: 'static>(mut self, mut f: Box<dyn FnMut(T) -> U>) -> Cont<R, U> {
        Cont(Box::new(move |c| {
            let f_ref = &mut f;
            let mut opt: Option<Box<(dyn FnOnce(T) -> R)>> = Some(Box::new(move |value| {
                c.call(f_ref(value))
            }));
            self.0(OnceFnRef(&mut opt))
        }))
    }
}

impl<R: 'static, T> Cont<R, T> {
    fn run(&mut self, terminal: Box<dyn FnOnce(T) -> R>) -> R {
        let mut opt = Some(terminal);
        self.0(OnceFnRef(&mut opt))
    }
}

impl<R: 'static> Cont<R, R> {
    fn run_id(&mut self) -> R {
        // TODO: for some reason "id" can't be used here
        self.run(Box::new(|x| x))
    }
}

// Monadic when
impl<R> Cont<R, ()> {
    fn when(condition: bool, action: Cont<R, ()>) -> Cont<R, ()> {
        if condition {
            action
        } else {
            Cont::pure(())
        }
    }
}

fn validate_name<R>(name: String, exit: Box<dyn FnOnce(String) -> Cont<R, ()>>) -> Cont<R, ()> {
    Cont::when(name.is_empty(), exit("You forgot to tell me your name!".into()))
}

pub fn run() {
    std::process::exit(0);
}