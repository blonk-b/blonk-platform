use teloxide::types::ChatId;

#[derive(Debug)]
pub struct InvalidTransactionDataError {}

#[derive(Debug)]
pub struct TransactionData {
    pub chat_id: ChatId,
    pub label: String,
    pub value: String,
    pub authority_account: String,
    pub transaction_index: u32,
    pub threshold: u8,
    pub current_threshold: u8,
    pub reject_count: u8,
    pub blink_name: String,
}

impl From<&TransactionData> for String {
    fn from(transaction_data: &TransactionData) -> Self {
        let result = format!(
            "{},{},{},{},{},{},{},{},{}",
            transaction_data.chat_id,
            transaction_data.label,
            transaction_data.value,
            transaction_data.authority_account,
            transaction_data.transaction_index,
            transaction_data.threshold,
            transaction_data.current_threshold,
            transaction_data.reject_count,
            transaction_data.blink_name,
        );
        
        result
    }
}

impl TryFrom<String> for TransactionData {
    type Error = InvalidTransactionDataError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() != 9 {
            return Err(InvalidTransactionDataError {});
        }

        let chat_id_u64 = parts[0]
            .parse::<i64>()
            .map_err(|_| InvalidTransactionDataError {})?;
        let chat_id = ChatId(chat_id_u64);

        let label = String::from(parts[1].trim());

        let value = String::from(parts[2].trim());

        let authority_account = String::from(parts[3].trim());

        let transaction_index = parts[4]
            .parse::<u32>()
            .map_err(|_| InvalidTransactionDataError {})?;

        let threshold = parts[5]
            .parse::<u8>()
            .map_err(|_| InvalidTransactionDataError {})?;

        let current_threshold = parts[6]
            .parse::<u8>()
            .map_err(|_| InvalidTransactionDataError {})?;

        let reject_count = parts[7]
            .parse::<u8>()
            .map_err(|_| InvalidTransactionDataError {})?;

        let blink_name = String::from(parts[8].trim());

        Ok(TransactionData {
            chat_id,
            label,
            value,
            authority_account,
            transaction_index,
            threshold,
            current_threshold,
            reject_count,
            blink_name,
        })
    }
}