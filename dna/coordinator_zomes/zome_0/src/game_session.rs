use hdk::prelude::*;
use zome_0_integrity::{
    EntryTypes, GameParams, GameSession, LinkTypes, PlayerStats, SessionState, UnitEntryTypes,
    GAME_CODE_TO_SESSION_TAG, OWNER_SESSION_TAG,
};

use crate::{game_code::get_game_code_anchor, player_profile::get_player_profiles_for_game_code};

pub fn start_game_session_with_code(game_code: String) -> ExternResult<EntryHash> {
    let anchor = get_game_code_anchor(game_code.clone())?;
    let players = get_player_profiles_for_game_code(game_code)?;
    let player_keys = players.iter().map(|x| x.player_id.clone()).collect();
    let game_params = GameParams {
        regeneration_factor: 1.1,
        start_amount: 300,
        num_rounds: 3,
    };

    create_game_session(player_keys, game_params, anchor)
}

pub fn create_game_session(
    players: Vec<AgentPubKey>,
    game_params: GameParams,
    anchor: EntryHash,
) -> ExternResult<EntryHash> {
    let agent_info_owner = agent_info()?;
    let game_session = GameSession {
        owner: agent_info_owner.agent_initial_pubkey.clone(),
        status: SessionState::InProgress,
        game_params: game_params,
        players: players,
        scores: PlayerStats::new(),
        anchor: anchor.clone(),
    };
    create_entry(&EntryTypes::GameSession(game_session.clone()))?;
    let game_session_entry_hash = hash_entry(&EntryTypes::GameSession(game_session))?;

    create_link(
        agent_info_owner.agent_initial_pubkey.clone(),
        game_session_entry_hash.clone(),
        LinkTypes::SessionOwnerLink,
        LinkTag::new(String::from(OWNER_SESSION_TAG)),
    )?;

    create_link(
        anchor,
        game_session_entry_hash.clone(),
        LinkTypes::GameCodeSessionLink,
        LinkTag::new(String::from(GAME_CODE_TO_SESSION_TAG)),
    )?;

    Ok(game_session_entry_hash)
}

pub fn get_my_own_sessions_via_source_query() -> ExternResult<Vec<(EntryHash, GameSession)>> {
    let def_index = ScopedEntryDefIndex::try_from(UnitEntryTypes::GameSession)?;
    let filter = ChainQueryFilter::new()
        .include_entries(true)
        .entry_type(EntryType::App(AppEntryType {
            id: def_index.zome_type,
            zome_id: def_index.zome_id,
            visibility: EntryVisibility::Public,
        }));

    let list_of_records = query(filter)?;
    let mut list_of_tuples: Vec<(EntryHash, GameSession)> = vec![];
    for record in list_of_records {
        let entry_option = record.entry().to_app_option().map_err(|error| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Could not deserialize Record to GameSession: {}",
                error
            )))
        })?;
        let gs: GameSession = entry_option.ok_or(wasm_error!(WasmErrorInner::Guest(
            "The targeted entry is not GameSession".into(),
        )))?;
        let gs_hash = record
            .action()
            .entry_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "The targeted entry is not GameSession".into(),
            )))?;
        list_of_tuples.push((gs_hash.clone(), gs));
    }
    Ok(list_of_tuples)
}
