# Example of a block root with a Verkle state root

## Purpose

In order to foster the implementation of verkle trees in clients, and to facilitate client interoperability, this repository provides:
 * a sample block containing a verkle proof,
 * a utility that decodes this block, verifies it, and displays some information.

This README file and the associated block will be updated over time. Feedback is welcome, especially requests for clarification. Open an issue in this repository, and I will adress it ASAP!

## Block content

This is a standard RLP block containing 3 transactions, and an added `VerkleProof` field at the end of the block header. The proof is defined against the following pre-state:

![Verkle tree](/verkle.png)

**Note**: on this diagram, the leading zeroes have been drop to improve readability.

This is a simple state that contains 3 accounts:

| Account address                              | Account component | Tree key                                                           | Value                               |
| ---------------                              | ----------------- | --------                                                           | -----                               |
| `0x0000000000000000000000000000000000000000` | Version           | `78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd00` | 0                                   |
| `0x0000000000000000000000000000000000000000` | Balance           | `78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd01` | 2000000000000000999 (little endian) |
| `0x0000000000000000000000000000000000000000` | Nonce             | `78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd02` | 0                                   |
| `0x0000000000000000000000000000000000000000` | Code hash         | `78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd03` | (empty code hash)                   |
| `0x0000000000000000000000000000000000000000` | Code size         | `78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd04` | 0 (no code)                         |
| `0x0102030000000000000000000000000000000000` | Version           | `6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd00` | 0                                   |
| `0x0102030000000000000000000000000000000000` | Balance           | `6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd01` | 999(little endian)  |
| `0x0102030000000000000000000000000000000000` | Nonce             | `6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd02` | 0                                   |
| `0x0102030000000000000000000000000000000000` | Code hash         | `6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd03` | (empty code hash)                   |
| `0x0102030000000000000000000000000000000000` | Code size         | `6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd04` | 0 (no code)                         |
| `0x0071562b71999873DB5b286dF957af199Ec94617` | Version           | `a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d600` | 0                                   |
| `0x0071562b71999873DB5b286dF957af199Ec94617` | Balance           | `a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d601` | 999913024999998002 (little endian)  |
| `0x0071562b71999873DB5b286dF957af199Ec94617` | Nonce             | `a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d602` | 3                                   |
| `0x0071562b71999873DB5b286dF957af199Ec94617` | Code hash         | `a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d603` | (empty code hash)                   |
| `0x0071562b71999873DB5b286dF957af199Ec94617` | Code size         | `a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d604` | 0 (no code)                         |

The "tree key" value is obtained by calling the `get_tree_key*` family of functions. These are the values that are reported when dumping the block's rlp:

```
> rlpdump --noascii block2.rlp
[
  # Block header
  [
    452eec13f7f71652565aeeed4b755363c1cb75e9e1d009b122d40c52675f8934,
    1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347,
    0000000000000000000000000000000000000000,
    4577df614497da7ac2f1b4f361d7008036d96d4b8c389279abf5b7c55a35a821,
    5f25ec3493913aef80e3d1d99e653321be3db3b16c3c83b82e6081cdce66a55c,
    1d929202540d18476cdd9b3f62ef5659928d9a72ce3f57f38c58cc83777f1b54,
    00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    020000,
    02,		# Block number
    47e7c4,
    0588c3,
    14,
    "",
    0000000000000000000000000000000000000000000000000000000000000000,
    0000000000000000,
    2de8eac1,
   
    # The proof   
000000000600000008080a08080a04000000aef5560376842597497caaaaef2fd176fa0583f6a9a85ed6f6213fb36ddd55164b084aefe7c95d371d42ba690a950e9144b78b1be51b037cf2c7282d8d015bd766309eec949a662a6d12ee3fba98d5515a91f7965808f760b5b2069da272670ad24543030c25a08399df898b9f3f8f1e0efb9c96ffa79402fdfaab7e9c522e8cdf3d789325e7703553323229266693aec748aa8f5ec4ffdee871023036cf3350e5d55291f0a273ddb1e4624896aa1f09357091dafaae5e1492079d71b112bc5bae533b83aa9ca480b720cd081ac25e7afc94e8e0b307fb23d25f8485462622b5cc45eee1a2d2868714490ff6d8ff6d07e63e326bf3a8a34623d75291a896af85f26865d232a15aa965336a6ce176e07765aa62b4600e2ed7d0b3aaa29c6e27db1ee59e45d4f30a9712f6c855016fe2e73fcc27aca441a001a527e88775bd1f99c621a982cbd72292ed0f70e073a469876cc6ef370c844bb455dc7f142131979855abd267aeea841c105e3717866b4ab2e2d7d7e61d823fe875c5fac7a74a4865c42ab615998ec69f4bdda1389ed192ffa480ecc6b9d1bdb2e26fae2af651b2ddabbfa114e56b6ed3edb6e96939aa44070fb7db95ae6ede6b5cbbf441751530c5d6bed9c1684ae87e4b1e7dcc045be2cf500ba9a818db500eaefac93bc5ce83d473cab88efa2a450eb3dd6d343d043590318f7fc324b9ba4512d3e422864bb139c51c795cdc8da7728c88a05d7e350e1cfaabfcc786a6bc3c7c992926553e2cb9e0e6222807a8869d16ab90634fb70eeabd8e2ea9f42e5d095c7c44c6f7919ed5fe3b58ba78a715d7a10522506f0ad45bb8ba4a8e7d2b89af676ccb46e1974cc01b172d8e5c0ae30e4d9d2722d55bd5f6fc3c2799fae0845ed2f82272e632672b33c58137fc6a813b7f183ff482b718693e4523289f1412196e07fe7f127af2063114ec6c2872712ca79458c21a9624be920f33f3858a1a224f7378994b29ad06,
    
    # List of (key, values) in the witness
    [
      [
        1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128400,
        "",
      ],
      [
        1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128401,
        "",
      ],
      [
        1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128402,
        "",
      ],
      [
        1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128403,
        "",
      ],
      [
        1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128404,
        "",
      ],
      [
        1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128480,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8300,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8301,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8302,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8303,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8304,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8380,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8381,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8382,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8383,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8384,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8385,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8386,
        "",
      ],
      [
        340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8387,
        "",
      ],
      [
        695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf900,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf901,
        e703c84e676dc11b000000000000000000000000000000000000000000000000,
      ],
      [
        695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf902,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf903,
        c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
      ],
      [
        695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf904,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2800,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2801,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2802,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2803,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2840,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2880,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2881,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2882,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2883,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d288c,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a4,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a5,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a6,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a7,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a8,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a9,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28aa,
        "",
      ],
      [
        b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28ab,
        "",
      ],
      [
        f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b500,
        "",
      ],
      [
        f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b501,
        "",
      ],
      [
        f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b502,
        "",
      ],
      [
        f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b503,
        "",
      ],
      [
        f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b504,
        "",
      ],
      [
        f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927200,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927201,
        324269359967e00d000000000000000000000000000000000000000000000000,
      ],
      [
        f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927202,
        0300000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927203,
        c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
      ],
      [
        f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927204,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
    ],
  ],
  
  # Transaction list
  [
    [
      03,
      342770c0,
      85fc,
      0102030000000000000000000000000000000000,
      03e7,
      "",
      25,
      8c5ae2492597dde3dcf5aecb0cff6ba3860d57c540ac1bcca3b129a2562c9ea2,
      4b8277e7629b7f0c29b2f4f598f5947a1ac2051b032418f886bd88f67407f9d8,
    ],
    [
      04,
      342770c0,
      85fc,
      0000000000000000000000000000000000000000,
      03e7,
      "",
      25,
      dcdad59185394a03a5ab978320b09e0f2b5c5c0aeef5a5a15a0147816043830f,
      04fb20787e200354df3d6fa40615f6d43c851222c0a06fda8e36940f8086dab9,
    ],
    [
      05,
      342770c0,
      7850,
      0000000000000000000000000000000000000000,
      "",
      "",
      25,
      37d860df9bfdcdedc84ad76dc2281c330f925b02eeff90b63162067b33abae07,
      7d0f5b4341b449320c59529a44ac98582c7957611b723f1fc686b8a3801d88bf,
    ],
    [
      06,
      342770c0,
      2dc6c0,
      "",
      10,
      6060604052600a8060106000396000f360606040526008565b00,
      26,
      e909f28a02715713732d38899d8dfe97688ffa3dc7a96a5072b367bac35badcb,
      61e24f56eab4f791158b16ca771b7914d85d401f549618329624be3d546adef9,
    ],
    [
      07,
      342770c0,
      2dc6c0,
      "",
      20,
      60806040526040516100109061017b565b604051809103906000f08015801561002c573d6000803e3d6000fd5b506000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555034801561007857600080fd5b5060008067ffffffffffffffff8111156100955761009461024a565b5b6040519080825280601f01601f1916602001820160405280156100c75781602001600182028036833780820191505090505b50905060008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690506020600083833c81610101906101e3565b60405161010d90610187565b61011791906101a3565b604051809103906000f080158015610133573d6000803e3d6000fd5b50600160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505061029b565b60d58061046783390190565b6102068061053c83390190565b61019d816101d9565b82525050565b60006020820190506101b86000830184610194565b92915050565b6000819050602082019050919050565b600081519050919050565b6000819050919050565b60006101ee826101ce565b826101f8846101be565b905061020381610279565b925060208210156102435761023e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8360200360080261028e565b831692505b5050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b600061028582516101d9565b80915050919050565b600082821b905092915050565b6101bd806102aa6000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c8063f566852414610030575b600080fd5b61003861004e565b6040516100459190610146565b60405180910390f35b6000600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166381ca91d36040518163ffffffff1660e01b815260040160206040518083038186803b1580156100b857600080fd5b505afa1580156100cc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100f0919061010a565b905090565b60008151905061010481610170565b92915050565b6000602082840312156101205761011f61016b565b5b600061012e848285016100f5565b91505092915050565b61014081610161565b82525050565b600060208201905061015b6000830184610137565b92915050565b6000819050919050565b600080fd5b61017981610161565b811461018457600080fd5b5056fea2646970667358221220a6a0e11af79f176f9c421b7b12f441356b25f6489b83d38cc828a701720b41f164736f6c63430008070033608060405234801561001057600080fd5b5060b68061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c8063ab5ed15014602d575b600080fd5b60336047565b604051603e9190605d565b60405180910390f35b60006001905090565b6057816076565b82525050565b6000602082019050607060008301846050565b92915050565b600081905091905056fea26469706673582212203a14eb0d5cd07c277d3e24912f110ddda3e553245a99afc4eeefb2fbae5327aa64736f6c63430008070033608060405234801561001057600080fd5b5060405161020638038061020683398181016040528101906100329190610063565b60018160001c6100429190610090565b60008190555050610145565b60008151905061005d8161012e565b92915050565b60006020828403121561007957610078610129565b5b60006100878482850161004e565b91505092915050565b600061009b826100f0565b91506100a6836100f0565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff038211156100db576100da6100fa565b5b828201905092915050565b6000819050919050565b6000819050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600080fd5b610137816100e6565b811461014257600080fd5b50565b60b3806101536000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c806381ca91d314602d575b600080fd5b60336047565b604051603e9190605a565b60405180910390f35b60005481565b6054816073565b82525050565b6000602082019050606d6000830184604d565b92915050565b600081905091905056fea26469706673582212209bff7098a2f526de1ad499866f27d6d0d6f17b74a413036d6063ca6a0998ca4264736f6c63430008070033,
      26,
      66241a78c508f5786ee7778e264c2d55cf64e4036e8f17917e6db89666b2eec6,
      7b8f093a07a7a93e174c7dccd2a0833b5e9f608ba17b1f0d5a3da2a4164a0132,
    ],
  ],
  [],
]

```

## Verkle proof format

This section is subject to change, so make sure that you watch this description as it will be udpated.

The verkle tree update adds two extra fields for the block: a proof and a list of (key, values) representing the _pre_ state of a block.

| Field | Block Header index | Description | Format |
| ----- | :----------------: | ----------- | ------ |
| Proof | 16 | Binary payload to pass to `rust-verkle` directly:<br /> * len(Proof of absence stem) ++ Proof of absence stems <br /> * len(depths) ++ serialize(depthi ++ ext statusi) <br /> * len(commitments) ++ serialize(commitment)<br /> * Multipoint proof | Flat binary
| Key, values | 17 | For each (key, value): a list of two byte arrays. The first array is the 32-byte key, and the second array is a 32-byte value (if present) or an empty array (if absent) | RLP |

## Understanding the proof with the helper utility

The block that is provided, can be decyphered with the program provided in this repository.

```
> cargo run
(...)
de-serialized block:
- parent hash: 452eec13f7f71652565aeeed4b755363c1cb75e9e1d009b122d40c52675f8934
- storage root: 4577df614497da7ac2f1b4f361d7008036d96d4b8c389279abf5b7c55a35a821
- block number: 02
```

The program starts by dumping some information related to the block: it's block number two, and the parent's hash is the one that is seen in the RLP. It proceeds to dump the (key, value) list.

```
- key, value list:
	1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128400 is absent
	1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128401 is absent
	1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128402 is absent
	1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128403 is absent
	1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128404 is absent
	1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba04128480 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8300 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8301 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8302 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8303 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8304 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8380 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8381 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8382 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8383 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8384 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8385 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8386 is absent
	340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b8387 is absent
	695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf900 => 0000000000000000000000000000000000000000000000000000000000000000
	695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf901 => e703c84e676dc11b000000000000000000000000000000000000000000000000
	695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf902 => 0000000000000000000000000000000000000000000000000000000000000000
	695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf903 => c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
	695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf904 => 0000000000000000000000000000000000000000000000000000000000000000
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2800 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2801 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2802 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2803 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2840 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2880 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2881 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2882 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d2883 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d288c is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a4 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a5 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a6 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a7 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a8 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28a9 is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28aa is absent
	b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28ab is absent
	f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b500 is absent
	f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b501 is absent
	f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b502 is absent
	f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b503 is absent
	f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b504 is absent
	f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927200 => 0000000000000000000000000000000000000000000000000000000000000000
	f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927201 => 324269359967e00d000000000000000000000000000000000000000000000000
	f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927202 => 0300000000000000000000000000000000000000000000000000000000000000
	f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927203 => c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
	f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927204 => 0000000000000000000000000000000000000000000000000000000000000000
```

These are the same addresses that are present in the RLP dump above. Note that:
 * 3 transactions are value transfers from one account to the next, so their leaves follow the same pattern:
   * keys ending in `00`: an account version, which is always set to `0`
   * keys ending in `01`: an account balance
   * keys ending in `02`: an account nonce
   * keys ending in `03`: is the code hash of the account
   * keys ending in `04`: an the code size of the account
   * the 3 aforementioned transactions have a "stem" of either: `f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e9272` (pre-existing account), `695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf901` (pre-existing account), and `f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b500` (newly created account)
 * numbers are **little endian**-endcoded. For example, the address `f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927202` represents nonce `3` for an account, and the number 3 is the left-most byte.
 * "absent" values mean that, before block 2 was executed, these keys were missing and are set for the first time during the execution of the block.
 * 4 new accounts are created during the execution of this block, so there are 4 "groups" of absent values: 
   * `f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b5*` is a regular account, since no code location is declared as absent in the witness,
   * `340d7bcf5014d837c13b31e68fd8ce965fe36c7900aceac5a51187c2559b83*`, `1fa091c7b912595422432f4221cc364b81a80f31a3a7f7b6697e05ba041284*` and `b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28*` are contract creation, because the witness contains the proof that some "code" leaves were absent before the execution of the block. The range `0x80..0xff` is reserved for the first code chunks of a contract.
   * `b0eddfec049575da2cf70b958bdf65a735476327cee6af287a30b960ba6d28*` also sets some data upon creation (one u256 word) because the witness shows that a word at offset `0x40` was absent. The range `0x40..0x7f` is reserved for the first few words of a contract's storage.

The next step is to decode the proof from the RLP block. Some information is displayed on the screen:

```
Verkle proof:
 * verification hints: 1 1 1 1 1 1 None None Present None None Present 
 * commitments: aef5560376842597497caaaaef2fd176fa0583f6a9a85ed6f6213fb36ddd5516 4b084aefe7c95d371d42ba690a950e9144b78b1be51b037cf2c7282d8d015bd7 66309eec949a662a6d12ee3fba98d5515a91f7965808f760b5b2069da272670a d24543030c25a08399df898b9f3f8f1e0efb9c96ffa79402fdfaab7e9c522e8c 

```

The six `1` indicate that there are six "stems", and that they are at depth 1. There are two `Present`, indicating that two stems were present in the tree (corresponding to the two existing accounts). There are 4 `None` indicating that four stems were absent until this block, and they correspond to the four account creations. Then, the commitments used in the proof are listed.

The final step is to verify the proof:

```
Proof verified.
```

## Checking of a proof with the associated test

The rust code also includes a unit test `compare_with_geth`, that rebuilds the tree as provided by a unit test in geth, and generates the proof from the rust code. This is **NOT** the same tree as found in the example block. It is possible to print out the expected proof in hex format by typing:

```
> cargo test -- --nocapture
(...)
running 1 test
serialized proof=00000000030000000a0a0a0600000053acaf9df78f9714054fdaaa630d6a3e60521c57d301411ac4c49668df25e40c49a58622affc8f8cc93042c7d5977127823d382e2602f3dc3e79e984b5dcd1614e221a9dfb42671da7ad1018131752c994bcd0c7b01441777cc7e09f646abc06b309948924dfefdfc44792e4c9fce1cd86c9145e16970d6d20bf61139d7a5ee48dec49d7e5c2ec27f35dc664ca337c8d01bfe276b8eb4fbc24910a557cf2cd30e386a2666ccf5191dc9caf569f183375444b6f48ebcff55330dd63bcdfdee8243e16c371b13a368de5ab953f44ad91ccb3a11568ea70e1bd64a36c6f5792ff0167cef8b0cf5dcc222f9b97f86f7bfa012d9fa917ce34a505bb1126ff9e064a45d2ce33a4b04145c0ad64c33f2bbf62558c44679d734413e92f9431bb4f7bb49f2d22da9e9640ab2e0cce48f1430096b77343842ae4688e8c785715b7da12e15529673b6b232e861b49301908e476ee009ac3acf34ef9d97ef75e4bcd9ac2a7ef40d53c09abae5e2e302e22f3e66b7cd7e1f7d8adaacec845975be4b40fbe041b2aa44e8c5aff2e363c3cc24d76e4cb8f20a815bbd587b3b59ff1bac5d0053f8fe34bc4fbfa222420f5b34cd7986a01c1da8273a289ecd8d3e393a1ea7af40f71bd514394cef0a1af38cfcefcb7fe3aa78ad980ad56c657b9e1945cb2b5fd3d04830270ebf638a707c3be1efff67dc1a5b8251ae9a540d403f55dba146d9062ae6e70e639f9ee2bd7a7394954466277dd9c1ef256df31497892de34e8de0d788cf525e47669e8a959e5c20bb62583937b99c2a4b95dd37be1ce24dbf9a18eceaf370ce0da389cccd432ad46442e7c18c33f13e5e2d736a284bcbddca3cf85a2dd4bf049d89f950c0f1e03ef19f9729425e4ef15cb06dbe0a54fbee7e135e89892d90d06856951d4f4f9d5fe1ea41a4bc249e9fca9a7eb4c2a55ed4a427109c4807062af2fb0c8c10141cf1bd798181039285cf3ab88c13401d35404bb4a80541f1d200a3eb8264a060e58f56084b818f034fdda222193bdbc10b0e5eb6f92edd563cb87a9da7587f9625e4c9a06757f2a2ba13613121b8b693304c05dc2f0f80a
```

Breaking this down:
  * The first 4 bytes represent the value `0`, which means that there are no "proof-of-absence" stems, i.e. stems whose presence in the tree prove that one of the key we are missing is absent from the tree;
  * The next 4 bytes say that there are 3 different stems in the tree, which is correct;
  * for each of these stems, the "extension status" is `0x0a`; it means that each stem:
    * has a depth of 1, because the depth is obtained with `stem >> 3 = 0x0a >> 3 = 1`;
    * is present, because presence is determined by `stem & 0x3 == 2`, and `0x0a & 3 == 2`;
  * the next 4 bytes contain the little-endian, 32-byte value of `6`, which signals that there are 6 serialized commitments:
    * `53acaf9df78f9714054fdaaa630d6a3e60521c57d301411ac4c49668df25e40c`
    * `49a58622affc8f8cc93042c7d5977127823d382e2602f3dc3e79e984b5dcd161`
    * `4e221a9dfb42671da7ad1018131752c994bcd0c7b01441777cc7e09f646abc06`
    * `b309948924dfefdfc44792e4c9fce1cd86c9145e16970d6d20bf61139d7a5ee4`
    * `8dec49d7e5c2ec27f35dc664ca337c8d01bfe276b8eb4fbc24910a557cf2cd30`
    * `e386a2666ccf5191dc9caf569f183375444b6f48ebcff55330dd63bcdfdee824`
  * The remaining data represent the IPA proof, whose verification proves that the provided leaves are indeed part of the state tree.

## Changelog

  * 22.01.22 - Initial version
  * 27.01.22:
    * proof and key,values lists are embedded in two distinct fields of the block (indices 16 and 17, respectively)
    * fix a duplicaiton issue for the commitments
    * remove the root commitment from the list
    * sort commitments by path
  * 28.01.22:
    * the keys and values are now encoded in the block with RLP
  * 01.02.22:
    * Add a test to compare geth and rust-verkle outputs
  * 08.02.22:
    * Rust and geth verkle proofs check out in the test
  * 09.02.22:
    * block proof decoding completed
  * 03.03.22:
    * updated to a block creating a contract

## TODO

 * [x] RLP encoding of the structure encapsulating the proof and the (key, values) tuples
 * [ ] SSZ encoding of that same structure when the final format has been defined
 * [ ] Add post state to the block
