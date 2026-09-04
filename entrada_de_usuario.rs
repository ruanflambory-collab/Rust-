use std::io;

pub fn usuario(){
    println!("-----Digite seu nome-----");

    let mut lost=String::new();

    io::stdin().read_line(&mut lost).unwrap(); 

    println!("O que voce digitou {}", lost);
}