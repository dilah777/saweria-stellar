💰 SaweriaStellar dApp (Soroban Smart Contract Integrated)
📖 Application Description
SaweriaStellar is a decentralized application built on the Stellar network that empowers users to support creators or friends by sending tips seamlessly. This platform has been fully upgraded to include Soroban Smart Contract integration, allowing tip transactions and custom messages to be securely recorded directly on-chain.

Featuring Dual Network Support, the dApp natively defaults to the Stellar Testnet for risk-free testing and evaluation, while dynamically allowing users to switch to the Stellar Mainnet.

🎯 Project Focus & Category
Developed as a Web3 Tip Jar / Crowdfunding Page, this project demonstrates core fundamentals of Stellar ecosystem development:

Soroban Smart Contract Integration: Utilizes custom Rust-based smart contracts to execute on-chain functions (send_tip and get_tips) via StellarSdk.Contract.

Wallet Setup & Connection: Seamless connect/disconnect functionality using the Freighter wallet API.

Transaction Flow: Simulating and sending XLM transactions with on-chain messages on the Stellar network, providing comprehensive user feedback and valid transaction hashes.

📜 Smart Contract IDs
Testnet Contract ID (Active): CACMS33RQPIWCJ5Q6LAJEM5XYZRHGKFOISSV3OB6I4IV5JTILTOOOU3Z

Mainnet Contract ID: CD3WC5DF2JA3SAHCVISQ6R34KB4YJZDCDB5URHTSFSY555QQJLFIVPQL

✨ Features
Soroban Contract Execution: Directly interacts with the Soroban RPC to execute Write (send_tip) and Read (get_tips) contract functions.

Dual Network Capabilities: Users can seamlessly toggle between Stellar Testnet (Default) and Mainnet.

Wallet Authentication: Secure login mechanism utilizing the Freighter browser extension API.

Friendbot Integration: A built-in feature to automatically fund newly created testnet wallets with 10,000 XLM for testing purposes with a single click.

Real-time Status: Displays contract simulation and execution status with valid transaction hashes straight from the blockchain.

🛠️ Tech Stack
Frontend: HTML5, CSS3, Vanilla JavaScript.

Blockchain/Web3: Stellar SDK (@stellar/stellar-sdk), Freighter API, Soroban RPC.

Smart Contract: Rust (Soroban SDK).

🚀 Live Demo & Testing
Access the live application here: https://saweria-stellar.vercel.app

🎬 Video Demo
Watch the full demonstration of SaweriaStellar, including wallet connection, contract execution, and transaction tracking on the network:

Video Link: SaweriaStellar - Soroban Smart Contract Demo

📝 MVP Validation & User Feedback
As part of our commitment to building a user-centric dApp, we are actively collecting feedback on our Minimum Viable Product (MVP). If you have tested the application on the Stellar Testnet, please take 1 minute to share your experience!

Submit Feedback: SaweriaStellar User Feedback Form

🚀 How to Run Locally
Clone this repository.

Open the project folder in VS Code.

Use the Live Server extension to run the index.html file locally.

Ensure you have the Freighter Wallet extension installed on your browser and set to the appropriate network.
## 📸 Screenshots

### 1. Soroban Smart Contract Execution
![Transaction Success Hash](https://github.com/user-attachments/assets/257e8de5-0cdb-4e0b-83d7-f7bb04db3403)
*Description: Successful execution of the 'send_tip' Soroban smart contract on the Stellar Testnet, returning a verified transaction hash directly from the blockchain.*

### 2. Live Dashboard & Freighter Integration
![Live Dashboard](https://github.com/user-attachments/assets/95a7bc35-d49f-467a-a2a2-0065860312e6)
*Description: The updated frontend interface successfully reading and displaying the smart contract status after a transaction, seamlessly integrated with the Freighter wallet extension.*

---
*Built with passion, persistence, and lots of coffee by Muhammad Guntur Sa'dillah.*
