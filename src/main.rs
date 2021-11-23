use std::collections::HashMap;

use pokedex::graphql::get_pokemon::{get_pokemon_by_ids, GetPokemonByIds};
use pokedex::sprite::get_url;
use pokedex::translate::{search, translate_name, TranslateCode};

use alfred;
use anyhow::{Context, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<()> {
    let query = std::env::args().nth(1).unwrap();
    let ids = search(query, TranslateCode::Jp);

    let query = GetPokemonByIds::build_query(get_pokemon_by_ids::Variables { ids: Some(ids) });

    let client = reqwest::Client::new();

    let res = client
        .post("https://beta.pokeapi.co/graphql/v1beta")
        .json(&query)
        .send()
        .await?;

    let data: Response<get_pokemon_by_ids::ResponseData> = res.json().await?;
    if let Some(data) = data.data {
        let mut icons: HashMap<i64, String> = HashMap::new();
        for x in &data.pokemon_v2_pokemon {
            let path = save_image_file(&client, x.id).await?;
            icons.insert(x.id, path);
        }

        let items = data
            .pokemon_v2_pokemon
            .iter()
            .map(|x| {
                let mut builder = alfred::ItemBuilder::new("pokedex")
                    .arg(translate_name(x.id, TranslateCode::Jp).unwrap())
                    .title(translate_name(x.id, TranslateCode::Jp).unwrap())
                    .subtitle(x.id.to_string());

                if let Some(path) = icons.get(&x.id) {
                    builder.set_icon_path(path);
                }

                builder.into_item()
            })
            .collect::<Vec<_>>();

        return alfred::json::write_items(std::io::stdout(), &items)
            .with_context(|| "writing error");
    }

    Ok(())
}

async fn save_image_file(client: &reqwest::Client, id: i64) -> Result<String> {
    let path = format!("./icons/{}.png", id);
    if std::path::Path::new(&path).exists() {
        return Ok(path.clone());
    }

    let url = get_url(id);
    let res = client.get(url).send().await?;
    let bytes = res.bytes().await?;

    tokio::fs::create_dir_all("./icons").await?;
    let mut file = tokio::fs::File::create(&path).await?;
    file.write_all(&bytes).await?;

    Ok(path.clone())
}
