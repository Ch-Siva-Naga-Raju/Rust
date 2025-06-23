
fn main() {
    // Normal Iterator: Borrows ownership and doesn't mutate the vector
    let nums : Vec<i8>= vec![1,2,3,4,5];
    let nums_iter = nums.iter();
    let total : i8 = nums_iter.sum();
    println!("Sum of elements in the vector: {}", total);
    let nums_iter2 = nums.iter();
    let new_nums_iter2 = nums_iter2.filter(|x| **x > 3);
    for val in new_nums_iter2{
        println!("Value in the vector > 3 is : {}", val);
    }

    // Mutating Iterator: Borrows ownership and mutates the vector
    let mut vec = vec![1,2,3,4,5];
    let vec_iter = vec.iter_mut();
    for val in vec_iter {
        *val = *val * 2;
    }
    println!("The vector now is : {:?}", vec);

    // Into Iterator: Moves the ownership to the iterator
    let new_vec = vec![5,4,3,2,1];
    let new_vec_iter = new_vec.into_iter();
    for mut val in new_vec_iter{
        val = val + 10;
        print!("{},", val);
    }
    
}
