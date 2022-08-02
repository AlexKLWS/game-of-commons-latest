use hdi::prelude::*;

mod player_profile;
pub use player_profile::PlayerProfile;

mod constants;
pub use constants::GAME_CODES_ANCHOR;
pub use constants::PLAYER_LINK_TAG;

#[hdk_link_types]
pub enum LinkTypes {
    Anchor,
    PlayerLink,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(name = "player_profile", visibility = "public")]
    PlayerProfile(PlayerProfile),
}

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
