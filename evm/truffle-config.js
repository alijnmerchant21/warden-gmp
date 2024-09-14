const HDWalletProvider = require('@truffle/hdwallet-provider');
require('dotenv').config(); // To load environment variables

module.exports = {
  networks: {
    sepolia: {
      provider: () =>
        new HDWalletProvider(
          process.env.PRIVATE_KEY, // Private key from .env
          `https://sepolia.infura.io/v3/${process.env.INFURA_PROJECT_ID}` // Infura URL
        ),
      network_id: 11155111, // Sepolia's network ID
      gas: 5500000, // Gas limit
      confirmations: 2, // Confirmations to wait between deployments
      timeoutBlocks: 200, // Blocks before deployment times out
      skipDryRun: true, // Skip dry run before migrations
    },
  },
  compilers: {
    solc: {
      version: '0.8.20', // Match the Solidity version in your contract
      settings: {
        optimizer: {
          enabled: true,
          runs: 200,
        },
      },
    },
  },
};
