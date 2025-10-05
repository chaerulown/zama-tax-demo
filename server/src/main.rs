use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use concrete_shortint::{gen_keys, Ciphertext, ServerKey};
use concrete_shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

#[derive(Deserialize)]
struct TaxRequest {
    encrypted_salary: Vec<u8>, // bincode dari Ciphertext
}

#[derive(Serialize)]
struct TaxResponse {
    encrypted_tax: Vec<u8>, // hasil bincode
}

#[post("/calc_tax")]
async fn calc_tax(req: web::Json<TaxRequest>, key: web::Data<Arc<ServerKey>>) -> impl Responder {
    // Deser salary terenkripsi
    let mut enc_salary: Ciphertext = bincode::deserialize(&req.encrypted_salary).unwrap();

    // Hitung pajak 10% (pakai u8 dan mutable ref)
    let enc_tax = key.get_ref().smart_scalar_mul(&mut enc_salary, 10u8);

    let resp = TaxResponse {
        encrypted_tax: bincode::serialize(&enc_tax).unwrap(),
    };

    web::Json(resp)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (_client_key, sk) = gen_keys(PARAM_MESSAGE_2_CARRY_2);
    let server_key: Arc<ServerKey> = Arc::new(sk);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_key.clone()))
            .service(calc_tax)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
