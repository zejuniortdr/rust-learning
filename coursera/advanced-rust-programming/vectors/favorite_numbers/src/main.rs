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

    players.remove(1);
    println!("Players: {:?}", players);

    let player = players.pop();
    println!("Players: {:?}", players);
    println!("Player: {:?}", player);

    // Slicing
    let top3_movies = &favorite_movies[0..3];
    println!("Top 3 movies: {:?}", top3_movies);


}
