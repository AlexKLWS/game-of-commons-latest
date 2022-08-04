use hdk::prelude::*;
use zome_0_integrity::EntryTypes;
use zome_0_integrity::LinkTypes;
use zome_0_integrity::PlayerProfile;
use zome_0_integrity::PLAYER_LINK_TAG;

use crate::game_code::get_game_code_anchor;

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct JoinGameInfo {
    pub gamecode: String,
    pub nickname: String,
}

pub fn create_and_hash_entry_player_profile(nickname: String) -> ExternResult<EntryHash> {
    let agent = agent_info()?;
    debug!(
        "create_and_hash_entry_player_profile | nickname: {}, agent {:?}",
        nickname,
        agent.clone()
    );
    let player_profile = PlayerProfile {
        player_id: agent.agent_initial_pubkey,
        nickname,
    };
    create_entry(&EntryTypes::PlayerProfile(player_profile.clone()))?;
    hash_entry(&EntryTypes::PlayerProfile(player_profile))
}

pub fn join_game_with_code(input: JoinGameInfo) -> ExternResult<EntryHash> {
    let anchor = get_game_code_anchor(input.gamecode)?;
    let player_profile_entry_hash = create_and_hash_entry_player_profile(input.nickname)?;
    create_link(
        anchor.clone(),
        player_profile_entry_hash,
        LinkTypes::PlayerLink,
        LinkTag::new(String::from(PLAYER_LINK_TAG)),
    )?;
    Ok(anchor)
}

pub fn get_player_profiles_for_game_code(
    short_unique_code: String,
) -> ExternResult<Vec<PlayerProfile>> {
    let anchor = get_game_code_anchor(short_unique_code)?;
    let links = get_links(
        anchor,
        LinkTypes::PlayerLink,
        Some(LinkTag::new(String::from(PLAYER_LINK_TAG))),
    )?;
    let mut players = vec![];
    for link in links {
        debug!("link: {:?}", link);
        let record = get(link.target, GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Entry not found!".into())
        ))?;
        let entry_option = record.entry().to_app_option().map_err(|error| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Could not deserialize Record to PlayerProfile: {}",
                error
            )))
        })?;
        let entry: PlayerProfile = entry_option.ok_or(wasm_error!(WasmErrorInner::Guest(
            "The target entry is not agent pub key!".into()
        )))?;
        players.push(entry);
    }
    return Ok(players);
}
