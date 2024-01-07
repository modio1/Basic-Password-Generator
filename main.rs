use std::*;
use rand::*;
fn main(){
    println!("How many characters would you like the password to be? ");
    let mut pass_len = String::new();
    io::stdin().read_line(&mut pass_len).expect("Could not read line");
    let number: u32 = pass_len.trim().parse().expect("Not a valid number.");
    let pass = gen_pass(number);
    println!("Your password is {}",pass)
}

fn gen_pass(pass_len:u32) -> String {
    let mut counter:u32 = 0;
    let mut password = String::new();
    loop {
        let char_set = ["1","2","3","4","5","6","7","8","9","a","b","c","d","e","f","g","h",
        "i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F",
        "G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","!","@","#","$",
        "%","^","&","*","?"];
        let rand_index = rand::thread_rng().gen_range(0..=char_set.len() - 1);


        if counter >= pass_len {
            return password;
        }
        else {
            password = password + char_set[rand_index];
            counter = counter + 1;
            continue
        }

    }
}