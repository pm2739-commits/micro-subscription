#Micro-Subscription
- Name: Prityush
- Field: CSE (Cybersecurity)
- Passionate about blockchain security and decentralized applications
- Skilled in frontend–backend integration, smart contract deployment, and secure coding practices
- Advocate for open‑source collaboration and clear documentation
- Focused on production‑ready solutions and community impact
📖 Project Description
Micro Subscription is a blockchain‑based platform built on Stellar Soroban smart contracts. It enables users to deposit tokens, start subscriptions with customizable rates, and automatically deduct balances over time. The system ensures transparency, fairness, and automation, while giving users control to stop or check balances anytime. This project combines backend smart contract logic with a simple frontend interface for real‑world usability.
🌍 Vision
Micro Subscription envisions a future where digital services are accessible through seamless micro‑payments. By automating subscriptions on blockchain, it empowers creators and users with trustless, transparent, and efficient transactions. This project aims to reduce friction in digital economies, foster inclusivity, and create new opportunities for decentralized service models.
🛠 Development Plan
- Smart Contract Setup
- Functions: initialize, deposit, start, tick, stop, get_balance.
- Variables: balances, subscription details, token address.
- Business Logic
- Auto‑stop when balance runs out.
- Charge function deducts fees periodically.
- Frontend Integration
- HTML + JS interface with buttons for wallet connect, deposit, start, stop, balance.
- Styled UI for better usability.
- Wallet Connection
- Freighter wallet integration for signing transactions.
- Testing & Debugging
- Simulate and send transactions on Stellar Futurenet.
- Deployment
- Publish smart contract and frontend on GitHub for community use.
👤 Personal Story
I’m Prityush, a CSE (Cybersecurity) student passionate about building secure, transparent systems. My journey into blockchain showed me how micro‑subscriptions can empower both creators and users. This project reflects my drive to merge technical rigor with practical usability, and to share knowledge openly with the community.
⚙️ Installation Guide
Follow these steps to install and run the Micro Subscription project:
🔹 Prerequisites
- Freighter Wallet browser extension installed
- Stellar Futurenet account with test tokens (use the faucet to fund your wallet)
- Contract deployed on Futurenet (replace contract ID in app.js)
- Modern browser (Chrome, Firefox, Edge)

🔹 Steps
- Clone the Repository
git clone https://github.com/yourusername/MicroSubscription.git
cd MicroSubscription
- Frontend Setup
- Navigate to the frontend folder:
cd frontend
- Ensure index.html and app.js are present.
- Configure Contract ID
- Open app.js.
- Replace the placeholder with your deployed contract ID:
const contract = new Contract("YOUR_CONTRACT_ID_HERE");
- Run the Frontend
- Open index.html directly in your browser (double‑click or drag into browser window).
- Connect your Freighter Wallet when prompted.
- Interact with the Contract
- Use the buttons to:
- Start Subscription
- Deposit tokens
- Check Balance
- Stop Subscription
- Verify Transactions
- Go to Stellar Laboratory Futurenet Explorer (laboratory.stellar.org in Bing).
- Paste your contract ID to view state changes and transaction history.

🔹 Notes
- If your browser blocks the Soroban SDK CDN, download soroban-client.umd.js locally and reference it in index.html.
- Always ensure your wallet is connected to Futurenet, not Mainnet.
- Update app.js to use sendTransaction for real contract calls (instead of simulateTransaction).
<img width="403" height="391" alt="image" src="https://github.com/user-attachments/assets/5481537f-d1d8-415d-9c8f-85d82e9be8bc" />
