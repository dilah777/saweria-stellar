# 💰 Web3 Tipping dApp (Stellar Dual Network: Testnet & Mainnet)

## 📖 Application Description
**Web3 Tipping dApp** is a decentralized application built on the Stellar network that empowers users to support creators or friends by sending tips seamlessly. Upgraded from its initial version, this MVP (Minimum Viable Product) now features a fully functional **Dual Network Support**, allowing users to dynamically switch between the Stellar Testnet (for free, risk-free testing) and the Stellar Mainnet (for real XLM transactions).

Built with a focus on simplicity, commercial readiness, and user experience, this application allows users to authenticate using the Freighter wallet, request initial funds from the Stellar Friendbot (on Testnet), and execute raw XDR transactions directly to the respective Stellar Horizon network. This project showcases a functional end-to-end Web3 user flow utilizing a responsive CSS Grid dashboard, without relying on heavy frontend frameworks.

## 📜 Mainnet Smart Contract ID
As required for the mainnet deployment submission, below is the verified Smart Contract ID assigned to this project:
* **Contract ID:** `Q4BLBLYFQKTRUGNZYVWYC7BTEU3LSU35`

## ✨ Features
1. **Dual Network Capabilities:** Users can seamlessly toggle between Stellar Testnet and Mainnet. The UI dynamically isolates balances, transaction inputs, and transaction histories based on the active network selection.
2. **Wallet Authentication:** Secure login mechanism utilizing the Freighter browser extension API.
3. **Native Stellar Payments:** Users can send native XLM tokens to any valid Stellar public key on their chosen network.
4. **Friendbot Integration:** A built-in feature to automatically fund newly created testnet wallets with 10,000 XLM for testing purposes.
5. **Custom XDR Handling:** Bypasses standard SDK transaction submission by compiling and signing raw XDR base64 strings directly to the specific network's Horizon API.
6. **Real-time History:** Fetches and displays the most recent transactions (Sent/Received/Funded) directly from the Horizon API.

## 🛠️ Tech Stack
* **Frontend:** HTML5, CSS3 (Grid Layout), Vanilla JavaScript.
* **Blockchain/Web3:** Stellar SDK (`@stellar/stellar-sdk`), Freighter API (`@stellar/freighter-api`).
* **Networks:** Stellar Testnet & Stellar Public Mainnet (Horizon API).

## 🚀 Live Demo
Access the live application here: [https://saweria-stellar.vercel.app](https://saweria-stellar.vercel.app)

## 🚀 How to Run Locally
1. Clone this repository.
2. Open the project folder in VS Code.
3. Use the **Live Server** extension to run the `index.html` file locally.
4. Ensure you have the **Freighter Wallet** extension installed on your browser and set to the appropriate network.

## 📸 Screenshots

### 1. Live Mainnet Dashboard
![Live Mainnet Dashboard](https://github.com/user-attachments/assets/84d1bb03-76fc-4264-9d9a-d5a41162da45)
*Description: The live deployed interface, with the default network set to Stellar Mainnet and the Smart Contract ID integrated.*

### 2. Dual Network & Friendbot Integration
![Testnet Funding Success](https://github.com/user-attachments/assets/b3d978ca-07f3-40ba-9d62-113e7e0d93ac)
*Description: Demonstration of the Testnet mode, showing a successful wallet funding operation via the integrated Stellar Friendbot.*

### 3. Transaction Success & History
![Transaction History](https://github.com/user-attachments/assets/0ef8355c-e142-43f1-99a2-ec43bbcdcc2a)
*Description: Successful tipping execution on the Stellar Network, verified by the Transaction Hash and reflected in the Recent History tab.*

---
*Built with passion, persistence, and lots of coffee by Muhammad Guntur Sa'dillah.*