mod pb;
use pb::transfers::v1::{Transfer, Transfers};
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<Transfers, Error> {
    let mut transfers = Vec::new();

    for trx in block.transactions {
        let meta = match trx.meta {
            Some(m) => m,
            None => continue,
        };
        if meta.err.is_some() {
            continue;
        }

        let transaction = match trx.transaction {
            Some(t) => t,
            None => continue,
        };
        let message = match transaction.message {
            Some(m) => m,
            None => continue,
        };

        let mut account_keys: Vec<String> = message
            .account_keys
            .iter()
            .map(|k| bs58::encode(k).into_string())
            .collect();

        for k in &meta.loaded_writable_addresses {
            account_keys.push(bs58::encode(k).into_string());
        }
        for k in &meta.loaded_readonly_addresses {
            account_keys.push(bs58::encode(k).into_string());
        }

        for inst in message.instructions {
            let prog_idx = inst.program_id_index as usize;
            if prog_idx < account_keys.len() && account_keys[prog_idx] == SYSTEM_PROGRAM_ID {
                // System Program Transfer Instruction (index 2, 12 bytes total)
                if inst.data.len() == 12 && inst.data[0..4] == [2, 0, 0, 0] {
                    let amount = u64::from_le_bytes(inst.data[4..12].try_into().unwrap_or([0; 8]));
                    if inst.accounts.len() >= 2 {
                        let from_idx = inst.accounts[0] as usize;
                        let to_idx = inst.accounts[1] as usize;
                        if from_idx < account_keys.len() && to_idx < account_keys.len() {
                            transfers.push(Transfer {
                                from: account_keys[from_idx].clone(),
                                to: account_keys[to_idx].clone(),
                                amount,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(Transfers { transfers })
}
