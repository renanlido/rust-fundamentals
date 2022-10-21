fn main() {
    println!("Formatting");

    println!("");

    println!("Tupla");
    println!("{:?}", ("Renan", 33, true)); // tupla

    println!("");

    println!("Argumentos incorporados");
    println!(
        "My name is {name} and I am {age}",
        name = "Renan",
        age = "33"
    );

    println!("");

    println!("Placeholders");
    println!("My best friend is {0}. {0} loves {1}", "Uel", "Rafa");

    println!("");

    println!("Formatação Básica");
    println!("My name is {} and I am {}", "Renan", 33);

    println!("");

    //println!(4); //doesn't work this form
    println!("{}", 4); //it's work

    println!("");

    println!("Math");

    println!("");

    println!("Produz um binário equivalente a um inteiro");
    println!("Binary: {:b}", 10);

    println!("");

    println!("Adição");
    println!("12 + 3 = {}", 12 + 3); // runs with * - / +

    println!("");

    println!("Variáveis");

    println!("");

    println!("Imutáveis");

    let x: i32 = 5;

    println!("{}", x);

    println!("");

    println!("Mutáveis");

    let mut x: i32 = 5;

    x = x + 5;

    println!("{}", x);

    println!("Outra forma de reassinar variáveis");

    let x: i32 = 5;
    println!("{}", x);

    let x: i32 = 50;
    println!("{}", x);

    println!("");

    println!("Multiplas variáveis");

    let (a, b) = ("oi", "tudo bem");

    println!("{} {}", a, b);

    println!("");

    println!("Contantes");

    const ID: i32 = 50;

    println!("{}", ID);

    println!("");

    println!("Sombreamento / Variável de escopo - Shadowing");

    let mut b = 3;
    println!("{}", b);

    // O que vem aqui dentro não é alterado pelo que vem de fora. Só funciona alí dentro
    {
        let b = 5000;
        println!("{}", b);
    }

    b = b + 5;
    println!("{}", b);
}
