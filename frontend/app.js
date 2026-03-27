import { Server, Contract } from "soroban-client";

// Connect to Stellar Testnet RPC
const server = new Server("https://rpc-futurenet.stellar.org");
const contractId = "CAH5RM2NECTH4V5ON6UB3PR7CACCQVPZLZ6R67FOTHGTY33NRNTNUB2W";
const userAddress = "GDOPAKDW27FYBRNVE2FRWYZVQACRQCWALQV22GEK5V3WXBGZQRFEO2BA";

// Connect wallet (mock for now)
document.getElementById("connect").onclick = () => {
  document.getElementById("wallet").innerText = "Connected: " + userAddress;
};

// Start subscription
document.getElementById("start").onclick = async () => {
  const rate = document.getElementById("rate").value;
  const contract = new Contract(contractId);
  const tx = await server.simulateTransaction(contract.call("start", { user: userAddress, rate_per_sec: parseInt(rate) }));
  alert("Start subscription TX: " + JSON.stringify(tx));
};

// Deposit
document.getElementById("deposit").onclick = async () => {
  const amount = document.getElementById("amount").value;
  const contract = new Contract(contractId);
  const tx = await server.simulateTransaction(contract.call("deposit", { user: userAddress, amount: parseInt(amount) }));
  alert("Deposit TX: " + JSON.stringify(tx));
};

// Get balance
document.getElementById("balance").onclick = async () => {
  const contract = new Contract(contractId);
  const result = await server.simulateTransaction(contract.call("get_balance", { user: userAddress }));
  document.getElementById("balanceResult").innerText = "Balance: " + JSON.stringify(result);
};

// Stop subscription
document.getElementById("stop").onclick = async () => {
  const contract = new Contract(contractId);
  const tx = await server.simulateTransaction(contract.call("stop", { user: userAddress }));
  alert("Stop subscription TX: " + JSON.stringify(tx));
};