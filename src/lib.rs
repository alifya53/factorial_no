pub fn factor(num: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new(); // creates a new vector for the factors of the number
 
    for i in 1..((num as f32).sqrt() as i32 + 1) { 
        if num % i == 0 {
            factors.push(i); // pushes smallest factor to factors
            factors.push(num/i); // pushes largest factor to factors
        }
    }
    factors.sort(); // sorts the factors into numerical order for viewing purposes
    factors // returns the factors
}