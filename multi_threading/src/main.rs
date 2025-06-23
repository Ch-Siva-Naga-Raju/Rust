
use num_cpus;
use std::time::Duration;
use std::{ sync:: mpsc::{channel}, thread:: {self, spawn}};

fn test_basic_threading(){
    let handle = spawn(|| {
        for i in 1..10{
            println!("Spawn count {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main count {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap()
}

fn thread_message_passing(){
    //This gives random permutations of a set of words in the vector
    let (tx, rx) = channel();
    // let str_vals = vec![String::from("Hello"), String::from("Siva"), String::from("You"), String::from("are"), String::from("Great!")];
    for i in 0..5{
        let tx_clone = tx.clone();
        spawn(move || {
            let str_vals = vec![String::from("Hello"), String::from("Siva"), String::from("You"), String::from("are"), String::from("Great!")];
            let msg = str_vals[i].clone();
            tx_clone.send(msg).unwrap();
        });
    }
    drop(tx);
    let mut received = String::from("");
    for val in rx{
        received = received + " " + &val;
        println!("Value received is : {} and received is {}", val, received);
    }
    println!("Final received value is: {}", received);
}

fn sum_10_pow_8(){
    let (tx, rx) = channel();
    let mut ans:i64 = 0;
    println!("The number of logical cores on this machine are : {}", num_cpus::get());
    println!("The number of physical cores are : {}", num_cpus::get_physical());
    for i in 0..10{
        let tx_clone = tx.clone();
        let mut sum:i64 = 0;
        for j in 0..10000000{
            sum = sum + (10000000 * i as i64 + j);
        }
        tx_clone.send(sum).unwrap();
    }
    drop(tx);

    for val in rx{
        ans = ans + val;
    }
    println!("Value: {}", ans);
}

fn main() {
     thread_message_passing();
}
