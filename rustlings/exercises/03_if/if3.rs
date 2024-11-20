fn animal_habitat(animal: &str) -> &str {
    // DONE: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        // 2.0
        2
    } else if animal == "snake" {
        3
    } else {
        // "Unknown"
        // ^^^^^^^^^ expected integer, found `&str`
        // - `if` and `else` have incompatible types
        0
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
    println!("{}", animal_habitat("crab"));
    println!("{}", animal_habitat("gopher"));
    println!("{}", animal_habitat("snake"));
    println!("{}", animal_habitat("dinosaur"));
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
