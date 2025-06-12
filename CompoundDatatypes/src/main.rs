

fn main() {

    // Arrays
    let int_arr : [i8;5] = [1,2,3,4,5];
    println!("The array is {:?}", int_arr);
    let str_arr : [&str;4] = ["abc","cde", "adv", "ase"];
    println!("The stringArr is {:?}", str_arr);
    // let struct_arr : [Employee; 2] = [{
    //     employeeId="fjlasdfjal";
    //     name="slfjoasd";
    //     age=23;
    //     designation="Senior Mechanical Engineer";
    //     hobbies=["Eating", "Watching TV", "Reading Books"];
    // }, 
    // {
    //     employeeId="alskjdfao";
    //     name="sflaks";
    //     age=43;
    //     designation="Senior Mechanical Engineer";
    //     hobbies=["Eating", "Watching TV", "Reading Books"];
    // }];
    // println!("The Struct array is {:?}", struct_arr);
    
    //Tuples

    let person = ("Siva", 35, "Engineer", ["Watching Manga", "Watching News", "Sharing Information"]);
    println!("The tuple is {:?}", person);

    //Slices

    let slice = &[1,2,3,4,5];

    println!("This is a slice: {:?}", slice)
}
