use hdk::prelude::*;
use zome_0_integrity::{LinkTypes, GAME_CODES_ANCHOR};

pub fn create_game_code_anchor(short_unique_code: String) -> ExternResult<EntryHash> {
    let anchor = anchor(
        LinkTypes::Anchor,
        GAME_CODES_ANCHOR.into(),
        short_unique_code,
    )?;

    Ok(anchor)
}

pub fn get_game_code_anchor(game_code: String) -> ExternResult<EntryHash> {
    anchor(
        LinkTypes::Anchor,
        GAME_CODES_ANCHOR.into(),
        game_code.clone(),
    )
}
