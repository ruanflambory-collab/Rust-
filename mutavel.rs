// const 
//mutabilidade 

const UMA_HORA_EM_SEGUNDO: i32 = 1 * 60 * 60;

pub fn const01(){
    println!("====Inicio do Programa====");

    let mut x:i32 = 3;
    println!(" Meu numero x é {}", x);
    x = 5;

    println!(" Meu numero x agora  é {}", x);

    x = UMA_HORA_EM_SEGUNDO;

    println!(" Esse meu constante  {}", x);
}

