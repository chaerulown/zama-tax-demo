# 🏗️ Zama Tax Demo – FHE Final Project

This project is a **Fully Homomorphic Encryption (FHE) based tax calculation demo** built with Rust, Actix Web, and Zama’s Concrete/TFHE library.
The goal is to demonstrate how sensitive data (e.g., salary) can be processed while still encrypted, without ever being decrypted on the server side.

---

## ✨ Key Features

- 🔐 **Full FHE Encryption** – Salary data is encrypted on the client and processed on the server without decryption.
- 📡 **REST API** – Client ↔ server communication via Actix Web.
- 📊 **Dynamic Tax Calculation** – Tax percentage can be adjusted without changing the core logic.
- 🛠️ **Debug Logging** – Supports `info` and `debug` log levels for easier analysis.

---

## 📂 Project Structure

```
zama-tax-demo/
├── client/          # Client CLI app to send encrypted data
├── server/          # Actix REST API server to process encrypted tax
└── README.md        # Project documentation
```

---

## ⚙️ How to Run

### 1. Run the Server

Navigate into the `server` folder and run:

```bash
RUST_LOG=info cargo run -p server
```

The server will run on `http://127.0.0.1:8080`.

### 2. Run the Client

Navigate into the `client` folder and run:

```bash
RUST_LOG=info cargo run -p client
```

The client will:

1. Generate FHE keys.
2. Encrypt the salary input.
3. Send the encrypted data to the server via `/calc_tax`.
4. Receive the encrypted tax result from the server.
5. Decrypt the tax result and print it to the console.

---

## 🧮 Example Calculation

Input salary = `1000`, tax rate = `10%`

- Client encrypts `1000`.
- Server computes `1000 * 10 / 100 = 100`.
- Client decrypts the result → **100**.

Note: Calculation uses **integer scaling** (e.g., `salary * tax_rate / 100`) for efficiency under FHE.

---

## 🔍 Debug Logging

Use debug level to see detailed logs:

```bash
RUST_LOG=debug cargo run -p server
RUST_LOG=debug cargo run -p client
```

You will see logs for:

- Encryption/decryption process
- Encrypted JSON payloads
- Request/response flow

---

## 🚀 Roadmap

- [ ] Implement fully dynamic percentage input (flexible `tax_rate`).
- [ ] Optimize ciphertext size for better resource usage.
- [ ] Integrate into a simple blockchain use case (e.g., private balance check).
- [ ] Benchmark FHE performance vs non-FHE approach.

---

## 📜 License

MIT License – for educational and research purposes only.

---

🙏 Thank you,
Created by **Chaerul** as an FHE final project demo.
