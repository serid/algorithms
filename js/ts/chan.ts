type Fulfiller<T> = (value: T) => void;

function gen<T>(): [Promise<T>, Fulfiller<T>] {
    let resolver;

    let p = new Promise<T>((resolve, reject) => {
        resolver = resolve;
    });

    return [p, resolver];
}

function delay(n: number): Promise<void> {
    return new Promise<void>((resolve, reject) => {
        setTimeout(() => {
            resolve();
        }, n);
    })
}

class BufferedChannel<T> {
    vals: Array<T>;
    q: Array<Fulfiller<void>>;

    constructor() {
        this.vals = [];
        this.q = [];
    }

    send(v: T) {
        this.vals.push(v);

        if (this.q.length !== 0) {
            this.q.shift()();
        }
    }

    async receive(): Promise<T> {
        if (this.vals.length == 0) {
            let [p, r] = gen();
            this.q.push(r);
            await p;
        }

        return this.vals.shift();
    }
}

export async function test4() {
    let ch = new BufferedChannel();

    let t1 = async () => {
        for (const i of new Array(10)) {
            ch.send(100);
            ch.send(200);
            await delay(600);
        }
    };

    let t2 = async () => {
        for (const i of new Array(20)) {
            ch.send(1000);
        }
    };

    let t3 = async () => {
        while (true) {
            console.log(await ch.receive());
            // await delay(100);
        }
    };

    await Promise.all([t1(), t2(), t3()]);
}
