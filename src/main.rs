use csv::{ReaderBuilder, StringRecord};
use slug::slugify;
use std::env;
use std::error::Error;
use std::fmt;
use std::io::{self, Read};

// Tu som si definoval struct ktory ma reprezentovat CSV data so zahlavim a zaznamami
#[derive(Debug)]
struct Csv {
    headers: StringRecord,      // Tu ukladam zahlavia jednotlivych stlpcov
    records: Vec<StringRecord>, // Tu ukladam zaznamy, kazdy zaznam je reprezentovany StringRecord
}

impl Csv {
    // Tu si vytvaram novy CSV instanci
    fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        // Tu konfiguruju CSV reader aby odstrihol/trimol whitespace zo vsetkych poli
        let mut reader = ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(input.as_bytes());

        // Tu zas extraktujem a klonujem zahlavie, zbieram vsetky zaznamy do vectoru
        let headers = reader.headers()?.clone();
        let records: Vec<StringRecord> = reader.records().collect::<Result<_, _>>()?;

        // Uistujem sa, ze CSV ma zahlavie
        if headers.is_empty() {
            return Err("Empty CSV headers".into());
        }

        Ok(Csv { headers, records })
    }

    // Vypocitavam mamximalnu sirku potrebnu pre kazdy stlpec pre spravne formatovanie
    fn get_column_widths(&self) -> Vec<usize> {
        let mut widths = vec![0; self.headers.len()];

        // Hladam najvacsiu sirku potrebnu pre zahlavie
        for (i, header) in self.headers.iter().enumerate() {
            widths[i] = header.len();
        }

        // Tu porovnavam najdenu maximalnu sirku zahlavia s sirkou v datovych poliach
        for record in &self.records {
            for (i, field) in record.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(field.len());
                }
            }
        }

        widths
    }
}

// Implementacia zobrazenia formatovania pre CSV data
impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let widths = self.get_column_widths();

        // Formatovanie a print zahlavia
        for (i, header) in self.headers.iter().enumerate() {
            if i > 0 {
                write!(f, " | ")?; // Pridanie separatora mezi stlpce, aby to nejak vypadalo
            }
            write!(f, "{:width$}", header, width = widths[i])?;
        }
        writeln!(f)?;

        // Pridanie separator lajny pod zahlavie
        for (i, width) in widths.iter().enumerate() {
            if i > 0 {
                write!(f, "-+-")?;
            }
            write!(f, "{}", "-".repeat(*width))?;
        }
        writeln!(f)?;

        // Formatovanie a print datovych zaznamov
        for record in &self.records {
            for (i, field) in record.iter().enumerate() {
                if i > 0 {
                    write!(f, " | ")?;
                }
                if i < widths.len() {
                    write!(f, "{:width$}", field, width = widths[i])?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// tu su definovane samostatne fukncie pre textove transformacie
fn lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

fn uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

fn no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(" ", ""))
}

fn slugify_text(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(slugify(input)) // Tu konvertujem text na URL-friendly format
}

// Spracovanie CSV vstupu parsovanim a formatovanim
fn process_csv(input: &str) -> Result<String, Box<dyn Error>> {
    let csv = Csv::from_str(input)?;
    Ok(csv.to_string())
}

// Hlavna funkcia pre spracovanie textu ktora routuje do specifickyuch funkcii pre transformacie textu
fn process_text(operation: &str, input: &str) -> Result<String, Box<dyn Error>> {
    match operation {
        "lowercase" => lowercase(input),
        "uppercase" => uppercase(input),
        "no-spaces" => no_spaces(input),
        "slugify" => slugify_text(input),
        "csv" => process_csv(input),
        _ => Err("Invalid operation. Use: lowercase, uppercase, no-spaces, slugify, or csv".into()),
    }
}

// Main funckia - cita operacie z CLI argumentov a stdin vstupu
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Zaistenie radneho pouzitia s aspon jednym argumentom
    if args.len() < 2 {
        eprintln!(
            "Usage: {} [lowercase|uppercase|no-spaces|slugify|csv]",
            args[0]
        );
        std::process::exit(1);
    }

    // Citaj vstup z stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Zprocesuj text a pories errory
    match process_text(&args[1], &input) {
        Ok(result) => println!("{}", result),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }

    Ok(())
}
