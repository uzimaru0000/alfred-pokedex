pub struct GetPokemonById;
pub mod get_pokemon_by_id {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetPokemonById";
    pub const QUERY : & str = "query GetPokemonById($id: Int) {\n  pokemon_v2_pokemon_by_pk(id: $id) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n\nquery GetPokemonByIds($ids: [Int!]) {\n  pokemon_v2_pokemon(\n    where: { id: { _in: $ids } }\n    order_by: { id: asc }\n    limit: 10\n  ) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n\nquery GetPokemonByNames($names: [String!]) {\n  pokemon_v2_pokemon(\n    where: { name: { _in: $names } }\n    order_by: { id: asc }\n    limit: 10\n  ) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: Option<Int>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub pokemon_v2_pokemon_by_pk: Option<GetPokemonByIdPokemonV2PokemonByPk>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByIdPokemonV2PokemonByPk {
        pub id: Int,
        pub height: Option<Int>,
        pub weight: Option<Int>,
        pub pokemon_v2_pokemontypes: Vec<GetPokemonByIdPokemonV2PokemonByPkPokemonV2Pokemontypes>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByIdPokemonV2PokemonByPkPokemonV2Pokemontypes {
        pub pokemon_v2_type:
            Option<GetPokemonByIdPokemonV2PokemonByPkPokemonV2PokemontypesPokemonV2Type>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByIdPokemonV2PokemonByPkPokemonV2PokemontypesPokemonV2Type {
        pub id: Int,
        pub move_damage_class_id: Option<Int>,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for GetPokemonById {
    type Variables = get_pokemon_by_id::Variables;
    type ResponseData = get_pokemon_by_id::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_pokemon_by_id::QUERY,
            operation_name: get_pokemon_by_id::OPERATION_NAME,
        }
    }
}
pub struct GetPokemonByIds;
pub mod get_pokemon_by_ids {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetPokemonByIds";
    pub const QUERY : & str = "query GetPokemonById($id: Int) {\n  pokemon_v2_pokemon_by_pk(id: $id) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n\nquery GetPokemonByIds($ids: [Int!]) {\n  pokemon_v2_pokemon(\n    where: { id: { _in: $ids } }\n    order_by: { id: asc }\n    limit: 10\n  ) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n\nquery GetPokemonByNames($names: [String!]) {\n  pokemon_v2_pokemon(\n    where: { name: { _in: $names } }\n    order_by: { id: asc }\n    limit: 10\n  ) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub ids: Option<Vec<Int>>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub pokemon_v2_pokemon: Vec<GetPokemonByIdsPokemonV2Pokemon>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByIdsPokemonV2Pokemon {
        pub id: Int,
        pub height: Option<Int>,
        pub weight: Option<Int>,
        pub pokemon_v2_pokemontypes: Vec<GetPokemonByIdsPokemonV2PokemonPokemonV2Pokemontypes>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByIdsPokemonV2PokemonPokemonV2Pokemontypes {
        pub pokemon_v2_type:
            Option<GetPokemonByIdsPokemonV2PokemonPokemonV2PokemontypesPokemonV2Type>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByIdsPokemonV2PokemonPokemonV2PokemontypesPokemonV2Type {
        pub id: Int,
        pub move_damage_class_id: Option<Int>,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for GetPokemonByIds {
    type Variables = get_pokemon_by_ids::Variables;
    type ResponseData = get_pokemon_by_ids::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_pokemon_by_ids::QUERY,
            operation_name: get_pokemon_by_ids::OPERATION_NAME,
        }
    }
}
pub struct GetPokemonByNames;
pub mod get_pokemon_by_names {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetPokemonByNames";
    pub const QUERY : & str = "query GetPokemonById($id: Int) {\n  pokemon_v2_pokemon_by_pk(id: $id) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n\nquery GetPokemonByIds($ids: [Int!]) {\n  pokemon_v2_pokemon(\n    where: { id: { _in: $ids } }\n    order_by: { id: asc }\n    limit: 10\n  ) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n\nquery GetPokemonByNames($names: [String!]) {\n  pokemon_v2_pokemon(\n    where: { name: { _in: $names } }\n    order_by: { id: asc }\n    limit: 10\n  ) {\n    id\n    height\n    weight\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        id\n        move_damage_class_id\n        name\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub names: Option<Vec<String>>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub pokemon_v2_pokemon: Vec<GetPokemonByNamesPokemonV2Pokemon>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByNamesPokemonV2Pokemon {
        pub id: Int,
        pub height: Option<Int>,
        pub weight: Option<Int>,
        pub pokemon_v2_pokemontypes: Vec<GetPokemonByNamesPokemonV2PokemonPokemonV2Pokemontypes>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByNamesPokemonV2PokemonPokemonV2Pokemontypes {
        pub pokemon_v2_type:
            Option<GetPokemonByNamesPokemonV2PokemonPokemonV2PokemontypesPokemonV2Type>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPokemonByNamesPokemonV2PokemonPokemonV2PokemontypesPokemonV2Type {
        pub id: Int,
        pub move_damage_class_id: Option<Int>,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for GetPokemonByNames {
    type Variables = get_pokemon_by_names::Variables;
    type ResponseData = get_pokemon_by_names::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_pokemon_by_names::QUERY,
            operation_name: get_pokemon_by_names::OPERATION_NAME,
        }
    }
}
