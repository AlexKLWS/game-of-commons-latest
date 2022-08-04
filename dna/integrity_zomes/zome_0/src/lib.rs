use hdi::prelude::*;

mod player_profile;
pub use player_profile::PlayerProfile;

mod game_session;
pub use game_session::GameParams;
pub use game_session::GameSession;
pub use game_session::PlayerStats;
pub use game_session::ResourceAmount;
pub use game_session::SessionState;

mod constants;
pub use constants::GAME_CODES_ANCHOR;
pub use constants::GAME_CODE_TO_SESSION_TAG;
pub use constants::OWNER_SESSION_TAG;
pub use constants::PLAYER_LINK_TAG;

#[hdk_link_types]
pub enum LinkTypes {
    Anchor,
    PlayerLink,
    SessionOwnerLink,
    GameCodeSessionLink,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(name = "player_profile", visibility = "public")]
    PlayerProfile(PlayerProfile),
    #[entry_def(name = "game_session", visibility = "public")]
    GameSession(GameSession),
}

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
