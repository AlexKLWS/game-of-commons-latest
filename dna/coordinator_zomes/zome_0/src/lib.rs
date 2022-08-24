use hdk::prelude::*;
use zome_0_integrity::{GameSession, PlayerProfile};

mod game_code;
mod game_session;
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

#[hdk_extern]
pub fn start_game_session_with_code(game_code: String) -> ExternResult<EntryHash> {
    game_session::start_game_session_with_code(game_code)
}

#[hdk_extern]
pub fn get_my_owned_sessions(_: ()) -> ExternResult<Vec<(EntryHash, GameSession)>> {
    game_session::get_my_own_sessions_via_source_query()
}

#[hdk_extern]
pub fn get_all_game_codes(_: ()) -> ExternResult<Vec<String>> {
    game_code::get_all_game_codes()
}
