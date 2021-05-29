use std::time::Duration;

mod executor {
    use std::future::Future;
    use std::pin::Pin;
    use std::ptr::null;
    use std::task::{RawWaker, RawWakerVTable, Waker};
    use std::task::Context;
    use std::task::Poll;
    use std::thread::sleep;
    use std::time::Duration;

    static VT: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    fn new_raw_waker() -> RawWaker {
        RawWaker::new(null(), &VT)
    }

    unsafe fn clone(_d: *const ()) -> RawWaker {
        new_raw_waker()
    }

    unsafe fn wake(_d: *const ()) {}

    unsafe fn wake_by_ref(_d: *const ()) {}

    unsafe fn drop(_d: *const ()) {}

    fn new_waker() -> Waker {
        unsafe { Waker::from_raw(new_raw_waker()) }
    }

    type Task<T> = Pin<Box<dyn Future<Output=T>>>;

    pub struct Executor<T: Unpin> {
        tasks: Vec<Task<T>>,
        waker: Waker,
    }

    impl<T: Unpin> Executor<T> {
        pub fn new() -> Self {
            Executor {
                tasks: Vec::new(),
                waker: new_waker(),
            }
        }
    }

    impl<T: Unpin> Executor<T> {
        pub fn add(&mut self, f: Task<T>) {
            self.tasks.push(f);
        }

        pub fn run(&mut self) {
            let mut cx = Context::from_waker(&self.waker);

            while !self.tasks.is_empty() {
                let mut tasks_to_be_removed = Vec::new();

                for (i, task) in self.tasks.iter_mut().enumerate() {
                    println!("Polling");

                    match Pin::as_mut(task).poll(&mut cx) {
                        Poll::Pending => {},
                        Poll::Ready(_v) => {
                            tasks_to_be_removed.push(i);
                        },
                    }

                    sleep(Duration::from_millis(500));
                }

                // When a task is removed all remaining task indexes have to be fixed by 1
                let mut shift = 0;

                for i in tasks_to_be_removed {
                    self.tasks.remove(i - shift);
                    shift += 1;
                }
            }
        }
    }

    pub fn run<T>(f: impl Future<Output=T>) -> T {
        let wk = new_waker();
        let mut cx = Context::from_waker(&wk);

        let mut pin = Box::pin(f);

        loop {
            println!("Polling");

            let pin2 = Pin::as_mut(&mut pin);
            let r = pin2.poll(&mut cx);
            match r {
                Poll::Pending => (),
                Poll::Ready(v) => return v,
            }

            sleep(Duration::from_millis(500));
        }
    }
}

fn producer() -> impl std::future::Future<Output=i32> {
    async {
        100
    }
}

async fn doge() -> i32 {
    producer().await
}

mod delay {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::Context;
    use std::task::Poll;
    use std::time::{Duration, Instant};

    pub struct Delay {
        target_time: Instant,
    }

    impl Delay {
        pub fn new(d: Duration) -> Self {
            Delay {
                target_time: std::time::Instant::now() + d
            }
        }
    }

    impl Future for Delay {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if std::time::Instant::now() < self.target_time {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        }
    }
}

pub fn test() {
    let task1 = async {
        delay::Delay::new(Duration::from_millis(1500)).await;
        println!("Complete 1");
        delay::Delay::new(Duration::from_millis(1500)).await;
        println!("Complete 1");
    };
    let task2 = async {
        delay::Delay::new(Duration::from_millis(750)).await;
        println!("Complete 2");
        delay::Delay::new(Duration::from_millis(750)).await;
        println!("Complete 2");
    };

    // let ok = executor::run(task1);

    let mut ex = executor::Executor::new();

    ex.add(Box::pin(task1));
    ex.add(Box::pin(task2));

    ex.run();

    // println!("Result: {:?}", ok);
}