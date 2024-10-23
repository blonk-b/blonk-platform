#[derive(Debug)]
pub struct InvalidButtonMetadataError {}

#[derive(Debug)]
pub struct ButtonMetadata {
    pub transaction_id: i64,
    pub value: String,
}

impl From<&ButtonMetadata> for String {
    fn from(transaction_data: &ButtonMetadata) -> Self {
        let result = format!(
            "{},{}",
            transaction_data.transaction_id, transaction_data.value,
        );

        result
    }
}

impl TryFrom<String> for ButtonMetadata {
    type Error = InvalidButtonMetadataError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() != 2 {
            return Err(InvalidButtonMetadataError {});
        }

        let transaction_id = parts[0]
            .parse::<i64>()
            .map_err(|_| InvalidButtonMetadataError {})?;

        let value = String::from(parts[1].trim());

        Ok(ButtonMetadata {
            transaction_id,
            value,
        })
    }
}
