use anchor_lang::prelude::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

pub fn find_unique_account_metas_map(
    account_metas: &Vec<AccountMeta>,
) -> HashMap<Pubkey, AccountMeta> {
    let mut unique_account_metas_map: HashMap<Pubkey, AccountMeta> = HashMap::new();

    for account_meta in account_metas {
        let prev_account_meta = unique_account_metas_map.get(&account_meta.pubkey);

        if prev_account_meta.is_some()
            && prev_account_meta.unwrap().is_writable == account_meta.is_writable
        {
            continue;
        }

        unique_account_metas_map.insert(account_meta.pubkey, account_meta.clone());
    }

    unique_account_metas_map
}
