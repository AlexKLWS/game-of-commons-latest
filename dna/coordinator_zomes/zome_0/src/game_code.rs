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

pub fn get_all_game_codes() -> ExternResult<Vec<String>> {
    debug!("LOLOL");
    let path: Path = (&Anchor {
        anchor_type: GAME_CODES_ANCHOR.into(),
        anchor_text: None,
    })
        .into();
    let all_game_codes_anchor_hash = path.path_entry_hash()?;

    let links = get_links(all_game_codes_anchor_hash, LinkTypes::Anchor, None)?;
    let mut codes = vec![];
    for link in links {
        debug!("link: {:?}", link);
        let record = get(link.target, GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Entry not found!".into())
        ))?;

        let anchor: Anchor = record
            .entry()
            .to_app_option()
            .map_err(|error| {
                wasm_error!(WasmErrorInner::Guest(format!(
                    "Could not deserialize Record to PlayerProfile: {}",
                    error
                )))
            })?
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "The target entry is not agent pub key!".into()
            )))?;
        let code = anchor.anchor_type;
        codes.push(code);
    }

    return Ok(codes);
}
