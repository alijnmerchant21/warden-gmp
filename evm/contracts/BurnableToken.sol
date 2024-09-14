// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

/**
 * @title BurnableToken
 * @dev An ERC20 token that can be burned via cross-chain messages using the Axelar network.
 */

contract BurnableToken is AxelarExecutable, ERC20Burnable {
    // Event emitted when tokens are burned
    event TokensBurned(uint256 amount);

    /**
     * @dev Constructor that mints an initial supply of tokens to the contract itself.
     * @param gateway_ The address of the Axelar gateway contract.
     * @param initialSupply The initial supply of tokens to mint.
     */

    constructor(address gateway_, uint256 initialSupply) 
        AxelarExecutable(gateway_) 
        ERC20("AliTokens", "AM") 
    {
        // Mint the initial supply to the contract's address
        _mint(address(this), initialSupply);
    }

    /**
     * @dev Handles cross-chain messages received via Axelar.
     * @param sourceChain The name of the source chain.
     * @param sourceAddress The address of the source contract on the source chain.
     * @param payload The payload sent from the source chain, expected to be the amount to burn.
     */

    function _execute(
        string calldata sourceChain,
        string calldata sourceAddress,
        bytes calldata payload
    ) internal override {
        // Decode the payload to get the amount to burn
        uint256 amountToBurn = abi.decode(payload, (uint256));
        // Burn the specified amount of tokens
        burnTokens(amountToBurn);
    }

    /**
     * @dev Burns a specified amount of tokens from the contract's balance.
     * @param amount The amount of tokens to burn.
     */
    function burnTokens(uint256 amount) public {
        require(balanceOf(address(this)) >= amount, "Insufficient balance to burn");
        _burn(address(this), amount);
        emit TokensBurned(amount);
    }

    // Allows the contract to receive native currency (e.g., ETH)
    receive() external payable {}
}
