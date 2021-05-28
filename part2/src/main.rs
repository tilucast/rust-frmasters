struct City {
    description: String,
    residents: u64,
    is_coastal: bool
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal
        }
    } else {
        City{
            description: format!("a *non-coastal* city of approximately {} residentes", residents),
            residents,
            is_coastal
        } 
        
    }
}

fn main() {
    let rustville: City = new_city(85_000, false); //panic!("👉 TODO call new_city here, with whatever arguments you like!");

    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
