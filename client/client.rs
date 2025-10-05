use concrete_shortint::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use bincode;

#[derive(Serialize)]
struct TaxRequest {
    encrypted_salary: Vec<u8>,
}

#[derive(Deserialize)]
struct TaxResponse {
    encrypted_tax: Vec<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate key (sama param dengan server)
    let (client_key, server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2);

    let salary: u64 = 50;
    let enc_salary = client_key.encrypt(salary);
    let serialized = bincode::serialize(&enc_salary)?;

    let client = Client::new();
    let res: TaxResponse = client
        .post("http://127.0.0.1:8080/tax")
        .json(&TaxRequest { encrypted_salary: serialized })
        .send()?
        .json()?;

    let enc_tax: Ciphertext = bincode::deserialize(&res.encrypted_tax)?;
    let tax: u64 = client_key.decrypt(&enc_tax);
    println!("Gaji: {} → Pajak 10%: {}", salary, tax);

    Ok(())
}
