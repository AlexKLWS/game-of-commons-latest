use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct PlayerProfile {
    pub player_id: AgentPubKey,
    pub nickname: String,
}
