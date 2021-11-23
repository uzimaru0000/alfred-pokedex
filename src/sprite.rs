const ENDPOINT: &str = "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon";

pub fn get_url(id: i64) -> String {
    format!("{}/{}.png", ENDPOINT, id)
}
