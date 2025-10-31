const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying MockERC20Permit with account:", deployer.address);

  const USDT = await ethers.getContractFactory("MockERC20Permit", deployer);
  const usdt = await USDT.deploy(deployer.address);
  await usdt.waitForDeployment();

  console.log("MockERC20Permit deployed to:", usdt.target);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
