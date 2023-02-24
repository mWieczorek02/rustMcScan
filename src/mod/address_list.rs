use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct AddressList {
    address_list: Vec<String>,
}

pub fn get_address_list() -> Vec<String> {
    return serde_json::from_str::<AddressList>(
        &std::fs::read_to_string("data/address_list.json").expect("file not found"),
    )
    .unwrap()
    .address_list;
}
