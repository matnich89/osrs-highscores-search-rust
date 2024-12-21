# OSRS Highscores

A simple Rust library to retrieve Old School RuneScape (OSRS) Highscores for various game modes. This library leverages the official OSRS endpoints and uses [`ureq`](https://crates.io/crates/ureq) for making HTTP requests.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Raw Highscore Data Format](#raw-highscore-data-format)
- [Examples](#examples)
- [Data Structures](#data-structures)
- [Error Handling](#error-handling)
- [License](#license)

## Features

- Fetch **Standard** OSRS highscores  
- Fetch **Ironman** OSRS highscores  
- Fetch **Hardcore Ironman** OSRS highscores  
- Fetch **Ultimate Ironman** OSRS highscores  
- Parse the results into strongly typed structs (`Player`, `Stat`)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
osrs-highscores = "0.1.0"
```

*(If you are copying the code directly, ensure you also include `ureq` in your `Cargo.toml`. This library depends on [ureq](https://crates.io/crates/ureq).)*

```toml
[dependencies]
ureq = "2.12.1"
```

Then run:
```bash
cargo build
```

## Usage

```rust
use osrs_highscores::{
    standard_high_scores, 
    ironman_high_scores, 
    hardcode_high_scores, 
    ultimate_high_scores
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let standard_player = standard_high_scores("Zezima")?;
    println!("{:#?}", standard_player);

    let ironman_player = ironman_high_scores("IronMaiden")?;
    println!("{:#?}", ironman_player);

    let hardcore_player = hardcode_high_scores("HCIM_Legend")?;
    println!("{:#?}", hardcore_player);

    let ultimate_player = ultimate_high_scores("UIM_God")?;
    println!("{:#?}", ultimate_player);

    Ok(())
}
```

## API Reference

Below are the primary functions you can call:

| Function                          | Description                                                      |
|----------------------------------|------------------------------------------------------------------|
| `standard_high_scores(player)`    | Fetch the **Standard** OSRS Highscores for the given `player`.   |
| `ironman_high_scores(player)`     | Fetch the **Ironman** OSRS Highscores for the given `player`.    |
| `hardcode_high_scores(player)`    | Fetch the **Hardcore Ironman** OSRS Highscores for `player`.     |
| `ultimate_high_scores(player)`    | Fetch the **Ultimate Ironman** OSRS Highscores for `player`.     |

Each function returns a `Result<Player, Box<dyn std::error::Error>>`, where a successful result will contain a `Player` struct.

## Raw Highscore Data Format

The raw response from the official Old School RuneScape Highscores is essentially a CSV-like text with lines such as:
```
53479,1280,6406598
47916,65,452980
64661,53,136985
...
-1,-1
-1,-1
...
```
Each line typically contains `rank,level,xp` for a specific skill or minigame. Some lines might include “-1” values to indicate missing or irrelevant stats. Manually parsing this can be confusing since you need to know which line corresponds to which skill or activity. This library abstracts away that complexity by:
1. Making an HTTP request to the official endpoint.
2. Mapping each CSV line to a specific skill (e.g., Attack, Mining, etc.).
3. Returning a neatly structured `Player` object.

## Examples

```rust
use osrs_highscores::standard_high_scores;

fn main() {
    match standard_high_scores("Zezima") {
        Ok(player) => {
            println!("Found player: {:?}", player.name);
            for stat in player.stats {
                println!("{} => Level: {}, XP: {}", stat.skill, stat.level, stat.xp);
            }
        },
        Err(e) => {
            eprintln!("Error fetching player data: {}", e);
        }
    }
}
```

## Data Structures

### `Player`
Represents a single player's highscores. Contains:

```rust
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub stats: Vec<Stat>,
}
```

- **name**: The player's username
- **stats**: A list of `Stat` entries for the player's skills

### `Stat`
Represents a single skill's rank, level, and experience.

```rust
#[derive(Debug)]
pub struct Stat {
    pub skill: String,
    pub rank: i64,
    pub level: i64,
    pub xp: i64,
}
```

- **skill**: The skill name (e.g. `Attack`, `Mining`, `Cooking`)
- **rank**: The player's rank in that skill
- **level**: The player's level in that skill
- **xp**: The player's XP in that skill

## Error Handling

- If the player does not exist or is not found in the specified game mode, an error `"Player not found"` is returned.
- Other errors, such as network connectivity issues, are propagated as a `ureq::Error` or a `Box<dyn std::error::Error>`.

## License

This project is licensed under the [MIT License](./LICENSE).  
See the [LICENSE](./LICENSE) file for details.