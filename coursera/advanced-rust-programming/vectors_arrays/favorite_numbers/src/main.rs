#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let mut favorite_numbers = vec![8,28,5,25,10,6,11,9,2];

    println!("Numbers: {:?}", favorite_numbers);

    favorite_numbers.push(0);

    println!("Numbers: {:?}", favorite_numbers);

    favorite_numbers.sort();
    println!("Numbers: {:?}", favorite_numbers);


    let mut favorite_movies: Vec<&str> = Vec::new();
    favorite_movies.push("The Matrix");
    favorite_movies.push("Inception");
    favorite_movies.push("The Dark Knight");
    favorite_movies.push("Kill Bill");
    favorite_movies.push("Interstellar");

    println!("Movies: {:?}", favorite_movies);


    let mut players: Vec<Player> = Vec::new();
    players.push(Player { name: "Alice".to_string(), score: 100 });
    players.push(Player { name: "Bob".to_string(), score: 200 });
    players.push(Player { name: "Charlie".to_string(), score: 300 });

    println!("Players: {:?}", players);

    players.insert(2, Player { name: "Delta".to_string(), score: 250 });
    println!("Players: {:?}", players);
    for player in &players {
        println!("Player: {} Score {}", player.name, player.score);
    }

    players.remove(1);
    println!("Players: {:?}", players);

    let player = players.pop();
    println!("Players: {:?}", players);
    println!("Player: {:?}", player);

    // Slicing
    let top3_movies = &favorite_movies[0..3];
    println!("Top 3 movies: {:?}", top3_movies);


    // Arrays
    let movies_array = ["The Matrix", "Inception", "The Dark Knight", "Kill Bill", "Interstellar"];
    println!("Second Favorite Movie: {:?}", movies_array[1]);
    for movie in movies_array.iter() {
        println!("Movie: {}", movie);
    }


    // Matrix - Multi-dimensional arrays
    let matrix: [[i32;3]; 3]  = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    let five = matrix[1][1];
    println!("Value at (1,1): {}", five);
    for row in matrix.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
    }


    // Slices
    let movies_slice = &movies_array[2..5];
    println!("Movies Slice: {:?}", movies_slice);
    for movie in movies_slice.iter() {
        println!("Movie: {}", movie);
    }

    let slice_split = movies_slice.split(|&x| x == "Kill Bill");
    for (i, group) in slice_split.enumerate() {
        println!("Group {}: {:?}", i, group);
    }


    // Strings
    let mut s: &str = "Hello, World!";
    println!("Original String: {}", s);
    s = "Hello, Rust!";
    println!("Modified String: {}", s);
    println!("Length in bytes: {}", s.len());

    let s2 = String::from("Rust");
    let encoded = s2.as_bytes();
    println!("{:?}", encoded);

    for letter in s2.chars() {
        println!("{}", letter);
    }

    let h = String::from("Hello");

    let w = String::from("world");

    let u = format!("{}, {}!", h, w);
    println!("{}", u);
}
