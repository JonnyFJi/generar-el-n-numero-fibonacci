use std::io;

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let siguiente = a + b;
        a = b;
        b = siguiente;
    }
    b
}

fn main() {
    println!("Generador del n-ésimo número de Fibonacci");
    println!("Introduce n (0 para 0, 1 para 1, etc.):");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");

    let n: u64 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Número inválido. Usando n=10 como ejemplo.");
            10
        }
    };

    let resultado = fibonacci(n);
    println!("Fibonacci({n}) = {resultado}");
}
