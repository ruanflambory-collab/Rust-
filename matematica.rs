pub fn master(){

    let a = 10 ;
    let b = 3;

    println!("Essa é a Adição {}", a+b);
    println!("Essa é Subtração {}", a-b);
    println!("Essa é  Divisão{}", a/b);
    println!("Essa é  multiplicação{}", a*b);
    println!("Resto: {}", a % b);


    
    println!("-----Expoente------");

    let expoente: u32 = 3;
    let exp :u32 =2;
    println!(" O expeonte de 3 é {}", expoente.pow(exp));

}