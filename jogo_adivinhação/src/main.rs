use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("O numero secreto é: {secret_number}");

    println!("Por favor, insira seu palpite.");
    
    let mut numero = String::new();

    io::stdin()
        .read_line(&mut numero)
        .expect("Falha ao ler a linha");

    let numero: u32 = match numero.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, insira um número válido!");
            return;
        }
    };

    println!("Você adivinhou: {numero}");
    match numero.cmp(&secret_number) {
        Ordering::Less => println!("Muito pequeno!"),
        Ordering::Greater => println!("Muito grande!"),
        Ordering::Equal => println!("Você venceu!"),
    }
}
