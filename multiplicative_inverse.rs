use std::io::stdin;

fn main() {
    /* this is where the code runs
    * a modular multiplicative inverse of an integer a is an integer x,
    ? such that the product ax is congruent to 1 with respect to the modulus m.
    
    */
    // ? This is a prompt to get the first value A
    println!("Input A: ", );
    
    let mut input_text = String::new();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    // to convert the input to an integer
    let trimmed = input_text.trim();
    let x = trimmed.parse::<i64>() ;
    let a:i64 = x.unwrap();
    

    // This is to get the modulo reference 'm'
    println!("Input M: ", );
    let mut input_text2 =String::new();
    stdin()
        .read_line(&mut input_text2)
        .expect("failed to read from stdin");
    
    // coverting it from a string literal to integer
    let trimmed2 = input_text2.trim();
    let x2 = trimmed2.parse::<u32>() ;
    let m:u32 = x2.unwrap();

    // using 'a' and 'm' as input to calculate the modular multiplicative inverse
    let answer = modular_multiplicative_inverse(a, m);
    println!("The modulo inverse is: {}", answer );
}

/// Finds the modular multiplicative inverse of `a` modulo `m`
/// Returns the wrong result if `m` isn't prime.
fn modular_multiplicative_inverse(a: i64, m: u32) -> i64 {
    modular_pow(a, m - 2, m as _)
}

/// a recusive function to find the GCD of the two values
fn modular_pow(x: i64, exp: u32, modulo: i64) -> i64 {
    (match x.checked_pow(exp) {
        Some(x) => x,
        None => {
            let exp_a = exp / 2;
            let exp_b = exp - exp_a;
            modular_pow(x, exp_a, modulo) * modular_pow(x, exp_b, modulo)
        }
    }) % modulo
}