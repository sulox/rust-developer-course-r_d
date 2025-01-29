use slug::slugify;
use std::env;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Zbier CLI argumentov
    let args: Vec<String> = env::args().collect();

    // Kontrola ci bol argument dodany
    if args.len() < 2 {
        println!("Usage: {} [lowercase|uppercase|no-spaces|slugify]", args[0]);
        std::process::exit(1);
    }

    // Citanie z stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Zprocesovanie textu podla dodaneho argumentu
    let result = match args[1].as_str() {
        "lowercase" => input.to_lowercase(),
        "uppercase" => input.to_uppercase(),
        "no-spaces" => input.replace(" ", ""),
        "slugify" => slugify(&input),
        _ => {
            println!("Invalid argument. Use: lowercase, uppercase, no-spaces, or slugify");
            std::process::exit(1);
        }
    };

    // Print vysledku
    print!("{}", result);
    Ok(())
}

//priklad pouzitia
//cargo run -- [lowercase]
//Hello World
//[Stlac Ctrl+D]
//# Output: hello world
