use ureq::Response;

const STANDARD_HIGH_SCORES: &str = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";
const IRONMAN_HIGH_SCORES: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_ironman/index_lite.ws";
const HARDCORE_HIGH_SCORES: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_hardcore_ironman/index_lite.ws";
const ULTIMATE_HIGH_SCORES: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_hardcore_ultimate/index_lite.ws";

pub fn standard_high_scores(player: &str) -> Result<Player, Box<dyn std::error::Error>> {
    let url = format!("{}?player={}", STANDARD_HIGH_SCORES, player);
    make_request_handle_response(&url, player)
}

pub fn ironman_high_scores(player: &str) -> Result<Player, Box<dyn std::error::Error>> {
    let url = format!("{}?player={}", IRONMAN_HIGH_SCORES, player);
    make_request_handle_response(&url, player)
}

pub fn hardcode_high_scores(player: &str) -> Result<Player, Box<dyn std::error::Error>> {
    let url = format!("{}?player={}", HARDCORE_HIGH_SCORES, player);
    make_request_handle_response(&url, player)
}

pub fn ultimate_high_scores(player: &str) -> Result<Player, Box<dyn std::error::Error>> {
    let url = format!("{}?player={}", ULTIMATE_HIGH_SCORES, player);
    make_request_handle_response(&url, player)
}

fn make_request_handle_response(
    url: &str,
    player: &str,
) -> Result<Player, Box<dyn std::error::Error>> {
    match ureq::get(&url).call() {
        Ok(response) => Ok(get_stats_from_response(response, player)?),
        Err(ureq::Error::Status(404, _)) => Err("Player not found".into()),
        Err(e) => Err(e.into()),
    }
}

fn get_stats_from_response(
    response: Response,
    player: &str,
) -> Result<Player, Box<dyn std::error::Error>> {
    let body = response.into_string()?;
    let mut stats = Vec::new();

    let skill_names = [
        "Overall",
        "Attack",
        "Defence",
        "Strength",
        "Hitpoints",
        "Ranged",
        "Prayer",
        "Magic",
        "Cooking",
        "Woodcutting",
        "Fletching",
        "Fishing",
        "Firemaking",
        "Crafting",
        "Smithing",
        "Mining",
        "Herblore",
        "Agility",
        "Thieving",
        "Slayer",
        "Farming",
        "Runecrafting",
        "Hunter",
        "Construction",
    ];

    for (i, line) in body.lines().enumerate() {
        if i >= skill_names.len() {
            break;
        }

        let tokens: Vec<&str> = line.trim().split(',').collect();
        if tokens.len() == 3 {
            let stat = Stat {
                skill: skill_names[i].to_string(),
                rank: tokens[0].parse().unwrap_or(-1),
                level: tokens[1].parse().unwrap_or(-1),
                xp: tokens[2].parse().unwrap_or(-1),
            };
            stats.push(stat);
        }
    }

    Ok(Player {
        name: player.to_string(),
        stats,
    })
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub stats: Vec<Stat>,
}

#[derive(Debug)]
pub struct Stat {
    pub skill: String,
    pub rank: i64,
    pub level: i64,
    pub xp: i64,
}
