use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage, HumanAddr, StdResult, ReadonlyStorage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton, PrefixedStorage, ReadonlyPrefixedStorage};
use secret_toolkit::storage::{AppendStore, AppendStoreMut};


pub static CONFIG_KEY: &[u8] = b"config";
const PREFIX_TXS: &[u8] = b"transactions";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: CanonicalAddr,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
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

    pub fn store_message<S: Storage>(&self, store: &mut S, to: &HumanAddr) -> StdResult<()> {
        append_tx(store, &self, to)
    }

    pub fn get_messages<S: ReadonlyStorage>(
        storage: &S,
        for_address: &HumanAddr,
        page: u32,
        page_size: u32,
    ) -> StdResult<(Vec<Self>, u64)> {
        let store = ReadonlyPrefixedStorage::multilevel(
            &[PREFIX_TXS, for_address.to.0.as_bytes()],
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
            .map(|tx| tx.map(|tx| tx.into_humanized(api)).and_then(|x| x))
            .collect();
        txs.map(|txs| (txs, store.len() as u64))
    }
}

// #[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
// struct InternalTx {
//     pub from: HumanAddr,
//     pub message: String,
//     pub block_time: u64
// }
//
// impl From<&Tx> for InternalTx {
//     fn from(other: &Tx) -> Self {
//         Self {
//             from: other.from.clone(),
//             message: other.message.clone(),
//             block_time: other.block_time.clone()
//         }
//     }
// }

fn append_tx<S: Storage>(
    store: &mut S,
    tx: &Message,
    to: &HumanAddr
) -> StdResult<()> {
    let mut store = PrefixedStorage::multilevel(&[PREFIX_TXS, to.0.as_bytes()], store);
    let mut store = AppendStoreMut::attach_or_create(&mut store)?;

    // let int_tx = InternalTx::from(tx);

    store.push(&tx)
}

