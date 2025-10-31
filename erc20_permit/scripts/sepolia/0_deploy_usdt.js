const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying MockUSDT with account:", deployer.address);

  const USDT = await ethers.getContractFactory("MockUSDT");
  const usdt = await USDT.deploy();
  await usdt.waitForDeployment();

  console.log("MockUSDT deployed to:", usdt.target);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
