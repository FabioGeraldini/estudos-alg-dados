fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum_even_squares: i32 = numbers.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum();
    println!("Soma dos quadrados dos numeros pares: {}", sum_even_squares);
    let first_gt_five = numbers.iter().find(|&&x| x > 5);
    match first_gt_five {
        Some(x) => println!("Primeiro numero maior que 5: {}", x),
        None => println!("Nenhum numero maior que 5 encontroado."),
    }
}
