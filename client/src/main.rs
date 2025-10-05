use concrete_shortint::{
    Ciphertext, parameters::PARAM_MESSAGE_2_CARRY_2, gen_keys,
};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use bincode;
use log::{info, debug, error};
use env_logger;

#[derive(Serialize, Deserialize, Debug)]
struct TaxRequest {
    encrypted_salary: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaxResponse {
    encrypted_tax: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inisialisasi logger
    env_logger::init();
    info!("🚀 Client dimulai...");

    // Buat kunci enkripsi
    let (client_key, _server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2);
    info!("🔑 Kunci client berhasil dibuat");

    // Salary yang akan dienkripsi
    let salary: u64 = 1000;
    debug!("Salary sebelum enkripsi: {}", salary);

    // Enkripsi salary
    let enc_salary: Ciphertext = client_key.encrypt(salary);
    let serialized_salary = bincode::serialize(&enc_salary)?;
    debug!("Salary terenkripsi size={} bytes", serialized_salary.len());

    // Kirim ke server
    let req_body = TaxRequest {
        encrypted_salary: serialized_salary,
    };

    let client = Client::new();
    info!("📡 Mengirim salary terenkripsi ke server...");
    let resp = client.post("http://127.0.0.1:8080/calc_tax")
        .json(&req_body)
        .send()
        .await?;

    if resp.status().is_success() {
        let tax_resp: TaxResponse = resp.json().await?;
        debug!("Respon terenkripsi size={} bytes", tax_resp.encrypted_tax.len());

        // Deserialisasi hasil terenkripsi
        let enc_tax: Ciphertext = bincode::deserialize(&tax_resp.encrypted_tax)?;
        let tax_value: u64 = client_key.decrypt(&enc_tax);

        println!("💰 Pajak yang dihitung server: {}", tax_value);
        info!("✅ Pajak terdekripsi: {}", tax_value);
    } else {
        error!("Server error: {:?}", resp.status());
    }

    Ok(())
}
