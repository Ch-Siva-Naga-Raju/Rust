fn main() {
    nested_loops()
}

fn check_loop() -> () {
    let mut counter:u8 = 0;
    loop {
        counter += 1;
        if counter < 100 {
            println!("This is loop demo and the counter is {}", counter);
        }
    }
}

fn nested_loops() -> () {
    let mut counter = 0;
    'parent_loop: loop {
        loop {
            counter+=1;
            if counter < 10 {
                println!("{counter}");
            } else if counter == 25 {
                println!("The counter silently reached 25");
                break 'parent_loop;
            } else {
                break;
            }
        }
    }
}
