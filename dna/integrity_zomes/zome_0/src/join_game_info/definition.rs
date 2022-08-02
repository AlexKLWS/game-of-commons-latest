use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct JoinGameInfo {
    pub gamecode: String,
    pub nickname: String,
}
