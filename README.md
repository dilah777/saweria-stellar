# 💰 Web3 Tipping dApp (Stellar Dual Network: Testnet & Mainnet)

## 📖 Application Description
**Web3 Tipping dApp** is a decentralized application built on the Stellar network that empowers users to support creators or friends by sending tips seamlessly. Built with a focus on simplicity and user experience, this application allows users to authenticate using the Freighter wallet, request initial funds from the Stellar Friendbot, and execute transactions directly to the Stellar Horizon network.

This MVP (Minimum Viable Product) features a fully functional **Dual Network Support**, natively defaulting to the **Stellar Testnet** for risk-free testing and evaluation, while allowing users to dynamically switch to the Stellar Mainnet for real XLM transactions.

## 🎯 Project Focus & Category
This project is developed as a foundational **Tip Jar / Crowdfunding Page**. It focuses heavily on demonstrating the core fundamentals of Web3 development within the Stellar ecosystem, specifically:
* **Wallet Setup & Connection:** Implementing seamless connect/disconnect functionality using the Freighter wallet.
* **Balance Handling:** Fetching and clearly displaying the connected wallet’s native XLM balance in real-time.
* **Transaction Flow:** Sending XLM transactions on the Stellar Testnet with comprehensive user feedback, including success/failure states and transaction hashes.

This robust foundation paves the way for future scalability, including Soroban smart contract integration for real-time crowdfunding progress and live event synchronization.

## 📜 Mainnet Smart Contract ID
As part of the project's progression towards mainnet readiness, below is the verified Smart Contract ID assigned to this dApp:
* **Contract ID:** `CD3WC5DF2JA3SAHCVISQ6R34KB4YJZDCDB5URHTSFSY555QQJLFIVPQL`

## ✨ Features
1. **Dual Network Capabilities:** Users can seamlessly toggle between Stellar Testnet (Default) and Mainnet. The UI dynamically isolates balances, transaction inputs, and transaction histories based on the active network selection.
2. **Wallet Authentication:** Secure login mechanism utilizing the Freighter browser extension API.
3. **Native Stellar Payments:** Users can send native XLM tokens to any valid Stellar public key on their chosen network.
4. **Friendbot Integration:** A built-in feature to automatically fund newly created testnet wallets with 10,000 XLM for testing purposes with a single click.
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
### 1. Soroban Smart Contract Execution
![Transaction Success Hash](    <img width="1360" height="768" alt="image" src="https://github.com/user-attachments/assets/257e8de5-0cdb-4e0b-83d7-f7bb04db3403" />
      ) *Description: Successful execution of the 'send_tip' Soroban smart contract on the Stellar Testnet, returning a verified transaction hash directly from the blockchain.*  
      
### 2. Live Dashboard & Freighter Integration
![Live Dashboard](  <img width="1360" height="768" alt="image" src="https://github.com/user-attachments/assets/95a7bc35-d49f-467a-a2a2-0065860312e6" />
)
*Description: The updated frontend interface successfully reading and displaying the smart contract status after a transaction, seamlessly integrated with the Freighter wallet extension.*
      
---
*Built with passion, persistence, and lots of coffee by Muhammad Guntur Sa'dillah.*
