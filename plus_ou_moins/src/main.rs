use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    println!("Devinez le nombre !");
    println!("Merci d'entrer un nombre =)");
    let secret = rand::thread_rng().gen_range(1..101);
    loop
    {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Echec de la lecture entre user");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };
        println!("Votre nombre : {}", guess);
        match guess.cmp(&secret)
        {
            Ordering::Less => println!("C'est plus :/"),
            Ordering::Greater => println!("C'est moins :/"),
            Ordering::Equal => 
            {
                println!("Bravo tu as trouve le nombre secret !");
                break;
            }
        }
    }
}
