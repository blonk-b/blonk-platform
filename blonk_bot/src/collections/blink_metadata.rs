use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub label: String,
    pub name: String,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub label: String,
    pub href: String,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
pub struct BlinkMetadata {
    pub icon: String,
    pub title: String,
    pub description: String,
    pub label: String,
    pub links: Option<Links>,
    pub disabled: Option<bool>,
    pub error: Option<bool>,
}
