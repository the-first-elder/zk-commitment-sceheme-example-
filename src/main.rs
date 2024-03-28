use sha2::{Sha256, Digest};
use std::error::Error;
use std::fmt::{Debug, Display};

// Define the Auction trait
trait Auction {
    fn commit(userA: User, userB: User) -> Result<Vec<Vec<u8>>, Box<dyn Error>>;
    fn open(input: u32, commitment: Vec<Vec<u8>>) -> Result<bool, Box<dyn Error>>;

}

struct User {
    amount: u8,
}

impl Auction for User {
    fn commit(userA: User, userB: User) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let mut hasher = Sha256::new();
        let sum_of_numbers = userA.amount + userB.amount;
        hasher.update(format!("{}", sum_of_numbers).as_bytes());
        let result = hasher.finalize();
        Ok(vec![result.to_vec()])
    }
    
fn open(input: u32, commitment: Vec<Vec<u8>>) -> Result<bool, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}", input).as_bytes());
    let result = hasher.finalize();
    let result_vec = result.to_vec();

    // Flatten the commitment vector and compare it with the result vector
    let flattened_commitment = commitment.into_iter().flatten().collect::<Vec<u8>>();
    Ok(result_vec == flattened_commitment)
}


    
}

fn main() {
    println!("hello world");

    let userA = User { amount: 10 };
    let userB = User { amount: 11 };

    // Example usage of commit
    let commitment = match User::commit(userA, userB) {
        Ok(commitments) =>  commitments,
        Err(e) =>  {Vec::new()}
    };

    test_commit(22, commitment)

}
fn test_commit(input: u32, hashed: Vec<Vec<u8>>) {
    // Flatten the hashed vector and convert it to a single Vec<u8>
    let hashClone = hashed.clone();
    let flattened_hashed = hashed.into_iter().flatten().collect::<Vec<u8>>();

    // Convert the flattened hashed vector to a hexadecimal string for printing
    let hashed_hex = flattened_hashed.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("");

    match User::open(input, hashClone.clone()) {
        Ok(true) => print!("{} is a valid hash of {:?}", input, hashed_hex),
        Ok(false) => print!("{} is not a valid hash of {:?}", input, hashed_hex),
        Err(_) => println!("An error occurred."),
    }

    // Print the hashed vector as a hexadecimal string
    // println!("Hashed vector as bytes: {}", hashed_hex);
}


