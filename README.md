# warden-gmp
Example to demonstrate Warden &lt;> EVM using Axelar GMP

```bash
Starting migrations...
======================
> Network name:    'sepolia'
> Network id:      11155111
> Block gas limit: 30000000 (0x1c9c380)


2_deploy_contracts.js
=====================

   Deploying 'BurnableToken'
   -------------------------
   > transaction hash:    0x969021618f339d2e5231920652699b13071adb44fcce27cf7d46dca9e2dcba61
   > Blocks: 0            Seconds: 4
   > contract address:    0x5388dE880a16Ba9602746F3799E850E78148cd57
   > block number:        6688280
   > block timestamp:     1726294248
   > account:             0xc00d0c1255883B9c0D8D3a17927F5b8a06802937
   > balance:             0.371481630374370861
   > gas used:            950251 (0xe7feb)
   > gas price:           3.567435322 gwei
   > value sent:          0 ETH
   > total cost:          0.003389958982165822 ETH

   Pausing for 2 confirmations...

   -------------------------------
   > confirmation number: 1 (block: 6688281)
   > confirmation number: 2 (block: 6688282)
   > Saving artifacts
   -------------------------------------
   > Total cost:     0.003389958982165822 ETH

Summary
=======
> Total deployments:   1
> Final cost:          0.003389958982165822 ETH
```

## Examples

- [Cosmos-EVM](https://github.com/axelarnetwork/send-message-from-cosmos-to-evm-example)
- [Contract](https://sepolia.etherscan.io/token/0x5388de880a16ba9602746f3799e850e78148cd57?a=0x5388dE880a16Ba9602746F3799E850E78148cd57)
- [Call-Contract](https://github.com/axelarnetwork/axelar-examples/tree/main/examples/cosmos/call-contract)
- [Contract-Address](https://docs.axelar.dev/resources/contract-addresses/testnet/)
- [Send-Recieve](https://github.com/axelarnetwork/evm-cosmos-gmp-sample/tree/main/cosmwasm-integration/send-receive)
- [IBC-Channel](https://docs.axelar.dev/resources/contract-addresses/testnet/#ibc-channels)