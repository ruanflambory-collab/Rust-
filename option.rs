use std::io;
pub fn principal(){

println!("====Digite seu nome =====");

let mut nome =String::new();

io::stdin() .read_line(&mut nome).unwrap();


println!(" seja bem vindo {} é um prazer ter voce aqui ", nome);


println!("===Opçoes====");

println!("1- enviar  ");
println!("2- mostra seu nome");
println!("3- Mostra meu cargo");

println!("======Digite as opção=====");
let mut numero =String::new();

io::stdin().read_line(&mut numero).unwrap();

let numero01: i32 = numero.trim().parse().unwrap();

if numero01 <= 1{
    println!("Ok Sr(a){}  seu relatorio  foi enviado!!", nome);
}
else if numero01 <= 2{
    println!("Seu nome é {}", nome);
}
else if numero01 <= 3{
    println!("Sr(a){} seu cargo ainda esta indefinido", nome);

}

println!(" Digite a mensagem que quera recebe hoje!!");

println!("1- option  ");
println!("2- option");
println!("3- opition");
let mut usuario= String::new();


io::stdin().read_line(&mut usuario) .unwrap();

let usario01: i32 = usuario.trim().parse().unwrap();


match usario01{

    1=> println!(" Se um dia voce pensa em desistir pense/n que tem alguem que acredita em voce e se /n ispira em voce "),
    2=> println!("Deus te ama {}", nome),
    3=> println!(" Nunca é tarde pra tentar de novo!!"),
    _ =>println!("opção invalida"),
}











 


}