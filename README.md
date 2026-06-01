# 💰 Web3 Tipping dApp (Stellar Testnet)

## 📖 Application Description
**Web3 Tipping dApp** is a decentralized application built on the Stellar Testnet that empowers users to support creators or friends by sending tips seamlessly. Built with a focus on simplicity and user experience, this MVP (Minimum Viable Product) allows users to authenticate using the Freighter wallet, request initial funds from the Stellar Friendbot, and execute raw XDR transactions directly to the Stellar Horizon network. 

This project was developed as a submission for the Stellar Workshop and showcases a functional end-to-end Web3 user flow utilizing a responsive CSS Grid dashboard, without relying on heavy frontend frameworks.

## ✨ Features
1. **Wallet Authentication:** Secure login mechanism utilizing the Freighter browser extension API.
2. **Native Stellar Payments:** Users can send native XLM tokens to any valid Stellar public key.
3. **Friendbot Integration:** A built-in feature to automatically fund newly created testnet wallets with 10,000 XLM for seamless testing.
4. **Custom XDR Handling:** Bypasses standard SDK transaction submission by compiling and signing raw XDR base64 strings, proving a deep understanding of Stellar's transaction architecture.
5. **Real-time History:** Fetches and displays the most recent transactions (Sent/Received/Funded) directly from the Horizon API.

## 🛠️ Tech Stack
* **Frontend:** HTML5, CSS3 (Grid Layout), Vanilla JavaScript.
* **Blockchain/Web3:** Stellar SDK (`@stellar/stellar-sdk`), Freighter API (`@stellar/freighter-api`).
* **Network:** Stellar Testnet (Horizon API).

## 📜 Smart Contract ID
* **Contract/Asset Type:** Native XLM Asset Transfer 
* *(Note: This MVP focuses on native network capabilities via the Horizon API. Development for Soroban Smart Contract integration is planned for the mobile/APK version).*

## 🚀 How to Run
1. Clone this repository.
2. Open the project folder in VS Code.
3. Use the **Live Server** extension to run the `index.html` file.
4. Ensure you have the **Freighter Wallet** extension installed on your browser and set to the **Testnet** network.

## 📸 Screenshots

### 1. Dashboard & Wallet Connected
![Frontend Dashboard]([LINK_FOTO_1])
*Description: The main 3-column grid interface showing a successfully connected Freighter wallet and fetched XLM balance.*

### 2. Transaction Success & History
![Transaction Success]([LINK_FOTO_2])
*Description: Successful tipping execution showing the verified Transaction Hash returned by the Stellar Testnet, accurately reflected in the Recent History tab.*

---
*Built with passion, persistence, and lots of coffee by Muhammad Guntur Sa'dillah.*