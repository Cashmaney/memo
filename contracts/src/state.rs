use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage, HumanAddr, StdResult, ReadonlyStorage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton, PrefixedStorage, ReadonlyPrefixedStorage};
use secret_toolkit::storage::{AppendStore, AppendStoreMut};

const MAX_LENGTH: u16 = 280;
pub static CONFIG_KEY: &[u8] = b"config";
const PREFIX_MSGS: &[u8] = b"transactions";
pub const PERFIX_PERMITS: &str = "revoked_permits";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
    pub contract: HumanAddr
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}


#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Clone, Debug)]
pub struct Message {
    pub from: HumanAddr,
    pub message: String,
    pub block_time: u64
}

impl Message {
    pub fn new(from: HumanAddr, message: String, block_time: u64) -> Self {
        Self {
            from,
            message,
            block_time
        }
    }

    pub fn validate(&self) -> bool {
        return self.message.len() <= usize::from(MAX_LENGTH);
    }

    pub fn store_message<S: Storage>(&self, store: &mut S, to: &HumanAddr) -> StdResult<()> {
        append_msg(store, &self, to)
    }

    pub fn get_messages<S: ReadonlyStorage>(
        storage: &S,
        for_address: &HumanAddr,
        page: u32,
        page_size: u32,
    ) -> StdResult<(Vec<Self>, u64)> {
        let store = ReadonlyPrefixedStorage::multilevel(
            &[PREFIX_MSGS, for_address.0.as_bytes()],
            storage
        );

        // Try to access the storage of txs for the account.
        // If it doesn't exist yet, return an empty list of transfers.
        let store = AppendStore::<Message, _, _>::attach(&store);
        let store = if let Some(result) = store {
            result?
        } else {
            return Ok((vec![], 0));
        };

        // Take `page_size` txs starting from the latest tx, potentially skipping `page * page_size`
        // txs from the start.
        let tx_iter = store
            .iter()
            .rev()
            .skip((page * page_size) as _)
            .take(page_size as _);

        let txs: StdResult<Vec<Message>> = tx_iter
            .map(|tx| tx)
            .collect();
        txs.map(|txs| (txs, store.len() as u64))
    }
}

fn append_msg<S: Storage>(
    store: &mut S,
    msg: &Message,
    for_address: &HumanAddr,
) -> StdResult<()> {
    let mut store = PrefixedStorage::multilevel(&[PREFIX_MSGS, for_address.0.as_bytes()], store);
    let mut store = AppendStoreMut::attach_or_create(&mut store)?;
    store.push(msg)
}