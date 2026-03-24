use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FooterLink {
    label: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    title: String,
    css: String,
}
