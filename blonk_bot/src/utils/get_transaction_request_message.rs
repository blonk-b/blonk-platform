pub fn get_transaction_request_message(
    title: String,
    description: String,
    parameters: Option<String>,
    transaction_index: u32,
) -> String {
    let header = format!(
        "<b>New transaction request:</b> {}\n\n{}",
        title, description
    );

    let body = if let Some(parameters) = parameters {
        parameters
    } else {
        "".to_string()
    };

    let footer = format!("Transaction Nº{}", transaction_index);

    format!("{}\n\n{}\n{}", header, body, footer)
}
