use rand;
use std::{io, io::Write, sync::mpsc, thread, time};

const NUM: usize = 20;

fn sleep(x: u64) -> u64 {
    thread::sleep(time::Duration::from_millis(x));
    print!(".");
    let _ = io::stdout().flush();
    x
}

fn make_inputs() -> [u8; NUM] {
    let mut inputs: [u8; NUM] = [0; NUM];
    for i in 0..inputs.len() {
        inputs[i] = rand::random();
    }
    inputs
}

fn main() {
    let inputs = make_inputs();
    println!("inputs = {:?}", inputs);

    let (tx, rx) = mpsc::channel();

    let threads = inputs.map(|x| {
        let tx = tx.clone();
        thread::spawn(move || {
            let _ = tx.send(sleep(x.into()));
        })
    });

    let mut result: Vec<u64> = Vec::new();
    for _ in 0..inputs.len() {
        if let Ok(val) = rx.recv() {
            result.push(val);
        }
    }
    println!("\nresult = {:?}", result);

    for t in threads {
        let _ = t.join();
    }
}
