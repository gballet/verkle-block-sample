[![Rust](https://github.com/gballet/verkle-block-sample/actions/workflows/rust.yml/badge.svg)](https://github.com/gballet/verkle-block-sample/actions/workflows/rust.yml)

# Example of a block root with a Verkle state root

## Purpose

In order to foster the implementation of verkle trees in clients, and to facilitate client interoperability, this repository provides:
 * a sample block containing a verkle proof,
 * a utility that decodes this block, verifies it, and displays some information.

This README file and the associated block will be updated over time. Feedback is welcome, especially requests for clarification. Open an issue in this repository, and I will adress it ASAP!

## Block content

This is a standard RLP block containing 5 transactions, and an added `VerkleProof` field at the end of the block header. The proof is defined against the following pre-state:

![Verkle tree](/verkle.png)

**Note**: on this diagram, the trailing zeroes have been dropped to improve readability.

The subtree `0x87b1ab04d20704398264fcc43eb9e9b1839b1595fcb8e9cd3b8071694658bf*` will not be touched during the execution of this block, but the other two keys will.

The initial "pre" state contains 3 regular accounts, and no contract:

| Account address                              | Account component | Tree key                                                           | Value                               |
| ---------------                              | ----------------- | --------                                                           | -----                               |
| `0x0000000000000000000000000000000000000000` | Version           | `bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1900` | 0                                   |
| `0x0000000000000000000000000000000000000000` | Balance           | `bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1901` | 2000000000000000999 (little endian) |
| `0x0000000000000000000000000000000000000000` | Nonce             | `bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1902` | 0                                   |
| `0x0000000000000000000000000000000000000000` | Code hash         | `bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1903` | (empty code hash)                   |
| `0x0000000000000000000000000000000000000000` | Code size         | `bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1904` | 0 (no code)                         |
| `0x0002030000000000000000000000000000000000` | Version           | `87b1ab04d20704398264fcc43eb9e9b1839b1595fcb8e9cd3b8071694658bf00` | 0                                   |
| `0x0002030000000000000000000000000000000000` | Balance           | `87b1ab04d20704398264fcc43eb9e9b1839b1595fcb8e9cd3b8071694658bf01` | 999(little endian)  |
| `0x0002030000000000000000000000000000000000` | Nonce             | `87b1ab04d20704398264fcc43eb9e9b1839b1595fcb8e9cd3b8071694658bf02` | 0                                   |
| `0x0002030000000000000000000000000000000000` | Code hash         | `87b1ab04d20704398264fcc43eb9e9b1839b1595fcb8e9cd3b8071694658bf03` | (empty code hash)                   |
| `0x0002030000000000000000000000000000000000` | Code size         | `87b1ab04d20704398264fcc43eb9e9b1839b1595fcb8e9cd3b8071694658bf04` | 0 (no code)                         |
| `0x71562b71999873DB5b286dF957af199Ec94617f7` | Version           | `274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1600` | 0                                   |
| `0x71562b71999873DB5b286dF957af199Ec94617f7` | Balance           | `274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1601` | 999913024999998002 (little endian)  |
| `0x71562b71999873DB5b286dF957af199Ec94617f7` | Nonce             | `274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1602` | 3                                   |
| `0x71562b71999873DB5b286dF957af199Ec94617f7` | Code hash         | `274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1603` | (empty code hash)                   |
| `0x71562b71999873DB5b286dF957af199Ec94617f7` | Code size         | `274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1604` | 0 (no code)                         |

The "tree key" value is obtained by calling the `get_tree_key*` family of functions. These are the values that are reported when dumping the block's rlp:

```
> rlpdump --noascii block2.rlp
[
  # Block header
  [
    904e3f9205902a780563d861aaa9cd1d635597ad1893a92d7f83dc5fb51b6eb4,
    1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347,
    0000000000000000000000000000000000000000,
    350f40f771a73cd6bda4c37283b88c771179469b07633568b6047cf649b8c7d1,
    5f25ec3493913aef80e3d1d99e653321be3db3b16c3c83b82e6081cdce66a55c,
    8d7a148023d3a4612e85b2f142dcec65c358ab7fbd3aebdfef6868c018d44e3c,
    00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    020000,
    02, # Block number
    47e7c4,
    05802b,
    14,
    "",
    0000000000000000000000000000000000000000000000000000000000000000,
    0000000000000000,
    2de81128,

    # The proof   
    0000000006000000080a0a0808080400000055fc28df8d54aef6ffb43695ef3fed55993ae008dc033c36bb48d3efa131e7d85c3e125f152e0b296a8fc6a7506c52ea7cf364a85d2f38425c495845b8e5fcaa621c65eed175759bed2c1293ba153fcbcc03a77ec5b8be6a70525284fadfe5ce2c40ed9e5cc59ec79c1f64558e7712705119f5b6adba6abe67b0a85eea6a57a372012d816767142d06b30a45da766d3453d5eea9f411e6ee25ab672da1580d09606162ddc947c873020f4e251c671b680146bffe0ce1a26d6a426f00a3206a7a59be7ae8bf499671db45344db4de2d610c2e7788dadf3c3798ffdaba155028566f1499f496b9fd0507954835c425270f3b08c68ab5475b7fea7dcb4de1fabb9b2c7ef6dcb3858f66ef02925afb5fddc6be130d470a7613058d3bf176cb5fa53d1b778f4f1f4b3b176af9fd234be9844f065a650a3facc3f18550948ae0eb8cb92a48a0a0413bd051054e27e359834a584c63da39cc094deac2c2290f60c6066b400efec9f3cbb4428d5972c2829a1dbaab6363507e1b4391210e26d3a03ada6e3384cf4db8a53f35bc30fbb1015104658e94160f29ac7becd4e75bfa8e46864216069023ab2a2c12a2be1c8ceed81c05645b3c6f9023339af49e62626e5b890c710c6d4c15c0b0fc72bcf3e609c3687b85ebde41101b1a7487d64a0edaf56bb9125ab05310b17001004453ad8e7ec56a804a97f9f39218488fbdcb6d90a7a95f1a965420c7cda79b8436cac411c00996466b19cb4591f17bba2014d0d7de150e6042fec3a7a55f4cc4114b8b85322d6694abbedd1c2acad8a2a41908f60d9ee30ed851b6a47e5ad2fe9aa22d5c26e11c2e4e04a76f8ebb6d8a7b7e265e9a032463508bbf4f7304b5d353b65bb49b22abbc3316675c793171e3d5234a5f71526d32e70de0056d4c9225dc65c35adbc36c2eb184db805662cb10e9ecb34553b909b4ac2e49eb5558dcf2d6fe86a0bff9d7d21f57c78bd2ed82e125e78f2c2ea91b,

    # List of (key, values) in the witness
    [
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce00,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce01,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce02,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce03,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce04,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce80,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce81,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce82,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce83,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce84,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce85,
        "",
      ],
      [
        0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce86,
        "",
      ],
      [
        274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1600,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1601,
        32c649ae8d68e00d000000000000000000000000000000000000000000000000,
      ],
      [
        274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1602,
        0300000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1900,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1901,
        e703c84e676dc11b000000000000000000000000000000000000000000000000,
      ],
      [
        bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1902,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1903,
        c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
      ],
      [
        bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1904,
        "",
      ],
      [
        cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071400,
        "",
      ],
      [
        cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071401,
        "",
      ],
      [
        cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071402,
        "",
      ],
      [
        cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071403,
        "",
      ],
      [
        cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071404,
        "",
      ],
      [
        d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b00,
        "",
      ],
      [
        d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b01,
        "",
      ],
      [
        d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b02,
        "",
      ],
      [
        d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b03,
        "",
      ],
      [
        d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b04,
        "",
      ],
      [
        d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b80,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c00,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c01,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c02,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c03,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c40,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c80,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c81,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c82,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c83,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c8c,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca4,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca5,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca6,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca7,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca8,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca9,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771caa,
        "",
      ],
      [
        ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771cab,
        "",
      ],
    ],
  ],

  # Block body
  [
    # List of transactions
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
- parent hash: 904e3f9205902a780563d861aaa9cd1d635597ad1893a92d7f83dc5fb51b6eb4
- storage root: 350f40f771a73cd6bda4c37283b88c771179469b07633568b6047cf649b8c7d1
- block number: 02
```

The program starts by dumping some information related to the block: it's block number two, and the parent's hash is the one that is seen in the RLP. It proceeds to dump the (key, value) list.

```
- key, value list:
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce00 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce01 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce02 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce03 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce04 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce80 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce81 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce82 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce83 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce84 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce85 is absent
	0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce86 is absent
	274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1600 => 0000000000000000000000000000000000000000000000000000000000000000
	274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1601 => 32c649ae8d68e00d000000000000000000000000000000000000000000000000
	274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1602 => 0300000000000000000000000000000000000000000000000000000000000000
	bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1900 => 0000000000000000000000000000000000000000000000000000000000000000
	bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1901 => e703c84e676dc11b000000000000000000000000000000000000000000000000
	bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1902 => 0000000000000000000000000000000000000000000000000000000000000000
	bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1903 => c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
	bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1904 is absent
	cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071400 is absent
	cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071401 is absent
	cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071402 is absent
	cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071403 is absent
	cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071404 is absent
	d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b00 is absent
	d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b01 is absent
	d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b02 is absent
	d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b03 is absent
	d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b04 is absent
	d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b80 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c00 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c01 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c02 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c03 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c40 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c80 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c81 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c82 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c83 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c8c is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca4 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca5 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca6 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca7 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca8 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771ca9 is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771caa is absent
	ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771cab is absent
```

These are the same addresses that are present in the RLP dump above. Note that:
 * 3 transactions are value transfers from one account to the next, so their leaves follow the same pattern:
   * keys ending in `00`: an account version, which is always set to `0`
   * keys ending in `01`: an account balance
   * keys ending in `02`: an account nonce
   * keys ending in `03`: is the code hash of the account
   * keys ending in `04`: an the code size of the account
   * the 3 aforementioned transactions have a "stem" of either: `274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd16` (pre-existing account), `bf101a6e1c8e83c11bd203a582c7981b91097ec55cbd344ce09005c1f26d1901` (pre-existing account), and `f286609cc1a51fb9bd96dacc5be743d8cdcee2d58a9ea76688e8fa1096a3b500` (newly created account)
 * numbers are **little endian**-endcoded. For example, the address `274cde18dd9dbb04caf16ad5ee969c19fe6ca764d5688b5e1d419f4ac6cd1602` represents nonce `3` for an account, and the number 3 is the left-most byte.
 * "absent" values mean that, before block 2 was executed, these keys were missing and are set for the first time during the execution of the block.
 * 4 new accounts are created during the execution of this block, so there are 4 "groups" of absent values: 
   * `cac9a3e8dd152c9b5f8abcd254f1abe57d4acde35cfe0f919b43e6f093071*` is a regular account, since no code location is declared as absent in the witness,
   * `0785762a8d643f3892d163f783fe1d37e4e5cf63d2b08dff0dead8cdf0b7ce*`, `ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c*` and `d141d84155cf135593f0ac888e4af96c360bbc4d82dd9164311b3932ab9b9b*` are contract creation, because the witness contains the proof that some "code" leaves were absent before the execution of the block. The range `0x80..0xff` is reserved for the first code chunks of a contract.
   * `ddb1869fe308ddab3660375687fd2a3f94434c961ed68fc8beb6fc8459771c40*` also sets some data upon creation (one u256 word) because the witness shows that a word at offset `0x40` was absent. The range `0x40..0x7f` is reserved for the first few words of a contract's storage.
 * Note that since stem `0x87b1ab04d20704398264fcc43eb9e9b1839b1595fcb8e9cd3b8071694658bf*` wasn't touched, it is not present in the witness.

The next step is to decode the proof from the RLP block. Some information is displayed on the screen:

```
Verkle proof:
 * verification hints: 1 1 1 1 1 1 None Present Present None None None 
 * commitments: 55fc28df8d54aef6ffb43695ef3fed55993ae008dc033c36bb48d3efa131e7d8 5c3e125f152e0b296a8fc6a7506c52ea7cf364a85d2f38425c495845b8e5fcaa 621c65eed175759bed2c1293ba153fcbcc03a77ec5b8be6a70525284fadfe5ce 2c40ed9e5cc59ec79c1f64558e7712705119f5b6adba6abe67b0a85eea6a57a3
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
  * 25.11.22:
    * updated to banderwagon values
    * fixed a missing proof dump in utility

## TODO

 * [x] RLP encoding of the structure encapsulating the proof and the (key, values) tuples
 * [ ] SSZ encoding of that same structure when the final format has been defined
 * [ ] Add post state to the block
