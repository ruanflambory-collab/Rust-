 pub fn variaveis(){
    let a:bool = true;
    let b:bool = false;

    println!("Verdadeiro: {}", a);
    println!("Falso: {}", b);

    println!("3< 2 é {}", 3<2);
    println!("3>= 3 é {}", 3>=3);
    println!("3 == 3 é {}", 3==3);
    println!("3 != 3 é {}", 3!= 3);
    // && e logico 
    // || ou logico 

    println!(" 3>2 e 4>6 {}", 3>2 && 4>6);
    println!("3>2 ou 4<2{}", 3>2 || 4<2);
    
    let c:char = 'A';
    println!("{} é um caractere", c);
}