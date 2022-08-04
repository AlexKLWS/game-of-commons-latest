use hdi::prelude::*;
use std::collections::BTreeMap;

pub type ResourceAmount = i32;
pub type PlayerStats = BTreeMap<AgentPubKey, ResourceAmount>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SessionState {
    InProgress,
    Lost { last_round: EntryHash },
    Finished { last_round: EntryHash },
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct GameParams {
    pub regeneration_factor: f32,
    pub start_amount: ResourceAmount,
    pub num_rounds: u32,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct GameSession {
    pub owner: AgentPubKey,
    pub status: SessionState,
    pub game_params: GameParams,
    pub players: Vec<AgentPubKey>,
    pub scores: PlayerStats,
    pub anchor: EntryHash,
}
