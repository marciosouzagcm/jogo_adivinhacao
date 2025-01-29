use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Por favor, insira seu palpite.");

    let mut numero = String::new();

    io::stdin()
        .read_line(&mut numero)
        .expect("Falha ao ler a linha");

    println!("Você adivinhou: {}", numero);
}