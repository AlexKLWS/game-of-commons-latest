use hdk::prelude::*;
use zome_0_integrity::PlayerProfile;

mod game_code;
mod player_profile;

use crate::player_profile::JoinGameInfo;

#[hdk_extern]
pub fn create_game_code_anchor(short_unique_code: String) -> ExternResult<EntryHash> {
    game_code::create_game_code_anchor(short_unique_code)
}

#[hdk_extern]
pub fn join_game_with_code(input: JoinGameInfo) -> ExternResult<EntryHash> {
    player_profile::join_game_with_code(input)
}

#[hdk_extern]
pub fn get_players_for_game_code(short_unique_code: String) -> ExternResult<Vec<PlayerProfile>> {
    player_profile::get_player_profiles_for_game_code(short_unique_code)
}
