// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract MockERC20Permit is ERC20, ERC20Permit, Ownable {
    constructor(
        address initialOwner
    )
        ERC20("USDT_AiPay", "USDT")
        ERC20Permit("USDT_AiPay")
        Ownable(initialOwner)
    {
        _mint(initialOwner, 1000 * 10 ** decimals());
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
