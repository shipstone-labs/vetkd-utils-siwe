// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Test} from "forge-std/Test.sol";
import {IPDoc} from "src/IPDoc.sol";

contract IPDocTest is Test {
  IPDoc public instance;

  function setUp() public {
    address initialOwner = vm.addr(1);
    instance = new IPDoc(initialOwner);
  }

  function testUri() public view {
    assertEq(instance.uri(0), "");
  }
}
