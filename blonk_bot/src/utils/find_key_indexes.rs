use anchor_lang::prelude::AccountMeta;

pub fn find_key_indexes(ix_keys: &[AccountMeta], keys_unique: &[AccountMeta]) -> Vec<u8> {
    ix_keys
        .iter()
        .map(|a| {
            keys_unique
                .iter()
                .position(|k| k.pubkey == a.pubkey && k.is_writable == a.is_writable)
                .map_or(255, |index| index as u8) // Return -1 if not found, otherwise the index
        })
        .collect()
}
