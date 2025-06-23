trait GetSquare {
    fn square(&self) -> u32;
}

struct FamilyAge {
    father_age: u8,
    mother_age: u8,
    brother_age:u8,
    sister_age: u8
}
impl GetSquare for FamilyAge {
    fn square(&self) -> u32 {
        (self.brother_age as u32) * (self.brother_age as u32)
    }
}
fn main() {
    let my_family_ages = FamilyAge{
        father_age: 55,
        mother_age:50,
        brother_age:35,
        sister_age:36
    };
    println!("The square of age of my brother is : {}", my_family_ages.square());
}
