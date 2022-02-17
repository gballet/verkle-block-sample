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
  # Block Header
  [
    8b386883dd7b380608dec0c7e647214c92a57d5af291f32afe728e756a317ac8,
    1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347,
    0000000000000000000000000000000000000000,
    4084dfa306521c5e062e0bfb38921a3e31aea6964793fd58940da5551eaf1de4,
    6e9e81e95ca097bee7400db0b4942090566e69f84688a5f1c08a67fa4874ee72,
    055f69313d5d06f30d05e80adc946ee4202ac94d07f56df0b9b8f9ee481d9809,
    00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    020000,
    02,       # Block number
    47e7c4,
    018448,
    14,
    "",
    0000000000000000000000000000000000000000000000000000000000000000,
    0000000000000000,
    2de8eac1,
    
    # Serialized proof
00000000030000000a0a0a06000000b8ce3d5e8857cec668d54ec13353ec237ba776832452a28514800efd7589836649a58622affc8f8cc93042c7d5977127823d382e2602f3dc3e79e984b5dcd1619fa9e535b55a646a53dbee71d0182a52457558187d00aba5945c3865fccd51304b084aefe7c95d371d42ba690a950e9144b78b1be51b037cf2c7282d8d015bd7c0691f07dd31120f69b8d5d0b9adad2ef1da65b7f32e22fa8128980e43f1d23bd24543030c25a08399df898b9f3f8f1e0efb9c96ffa79402fdfaab7e9c522e8c86ab91e02de438bc61f7c4e688cd269ce53ba20d8d9f18c5d9341926192708e7d37359e660028843682a9247aed5b0692fa500f9e7a278025f7b363333aefde6e59071e4e69df5871f591515ef9a82b735509fd93ca002c4d3d20e2b413ae6cf67049dddccc8594e3a223246d7cc87df215b64b0b6e468da0e23f1c708b5fa8fd24d405ff0310be0c59a684aabbbf721f862f9d96516d340a4208c571deabe0f12d19183fbd72338ad47f144f97bb3f17229568a6d1101b56968f601be43ae3eacd53570b2ee589302c140732e7b83b045801e77a371c688a02069542c67384cabc47752f18e81e3f3c8011f35ef5aa13d9ca8ef048af60c67ea96fabff9782a68fc099557d898583fc730f52e7d72d7edc3b17d3c6376ee5e1ba7c9d13d74e64b7ed423c5230e17b10930e1bbb9082755e0068fdac0a0d5d97803542b4e3ddb3078ce79f9e1dd9f09ab7914ec3a9c4dc42423810ce16d2c582ec312d98bbfbc4f67e7cdf771fe9d8e0af86e6db9cbc46ce92663b7f8161abd3628a7ed5334724918cc2d6ba0a029c34ae21b0e5f36a57553672ece2ef5673675a46bf664c26f41f964d357e8cdab96570b9b24bf9809d8031af9a2d9b90cd671b98ef8db788a4d7ded9d52a052ea797585142f64f8627da04663361b88d6a70aac150b83f8cbec9def75da86e0fcbcdc88b3f3b78e0c9c4ec273f147b1db8c0f06ae2e102d86fd34d543bb5a00d2cbeed4f23fe03f1cfc338bbf479cb1e9fa1d2f5291f9de4df75225b34a87be749e4b831c9d05271a720e7b269b0f46011418602182be070d,

    # List of (key, values) of the pre-state, that are accessed during the
    # block's execution.
    [
      [
        6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd00,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd01,
        e703000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd02,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd03,
        c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
      ],
      [
        6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd04,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd00,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd01,
        e703c84e676dc11b000000000000000000000000000000000000000000000000,
      ],
      [
        78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd02,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd03,
        c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
      ],
      [
        78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd04,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d600,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d601,
        324269359967e00d000000000000000000000000000000000000000000000000,
      ],
      [
        a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d602,
        0300000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d603,
        c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
      ],
      [
        a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d604,
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
- parent hash: 2a0fa77c9673bac5696974f6693b917bd391bcbfe564e1d246ebaa9835255c8a
- storage root: 03aec4276033869b5900313c7d807cdaa948e38acd9aa43c98e05089adc7389d
- block number: 02
```

The program starts by dumping some information related to the block: it's block number two, and the parent's hash is the one that is seen in the RLP. It proceeds to dump the (key, value) list.

```
- key, value list:
	6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd00 => 0000000000000000000000000000000000000000000000000000000000000000
	6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd01 => e703000000000000000000000000000000000000000000000000000000000000
	6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd02 => 0000000000000000000000000000000000000000000000000000000000000000
	6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd03 => c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
	6c99a3a0427cab63b7ab24f0683da88a1c5ed53f7b072b9e4efebd5dc412fd04 => 0000000000000000000000000000000000000000000000000000000000000000
	78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd00 => 0000000000000000000000000000000000000000000000000000000000000000
	78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd01 => eb65f1efab70c11b000000000000000000000000000000000000000000000000
	78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd02 => 0000000000000000000000000000000000000000000000000000000000000000
	78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd03 => c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
	78fdaed8ac3619d9b9520a66b57fdc16a1ed03b29ab8e9fb021c39bf8cefdd04 => 0000000000000000000000000000000000000000000000000000000000000000
	a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d600 => 0000000000000000000000000000000000000000000000000000000000000000
	a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d601 => 328502614d4ce00d000000000000000000000000000000000000000000000000
	a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d602 => 0300000000000000000000000000000000000000000000000000000000000000
	a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d603 is absent
	a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d604 is absent
```

These are the same addresses that are present in the RLP dump above. Note that:
 * no contract is being called, these transactions are just value transfers from one account to the next, so all leaves are either:
   * keys ending in `00`: an account version, which is always set to `0`
   * keys ending in `01`: an account balance
   * keys ending in `02`: an account nonce
   * keys ending in `03`: is the code hash of the account
   * keys ending in `04`: an the code size of the account
 * numbers are **little endian**-endcoded. For example, the address `a365db4f33df4f95bf2ae41da5a1bc3c804c3e511e7fddff4eabd000b5c0d602` represents nonce `3` for an account, and the number 3 is the left-most byte.
 * "absent" values mean that, before block 2 was executed, these keys were missing. This means that prior to the block execution, the value for the hash and the size of this account's code were not present in the tree. _This is a bug in the code that produced this block, the account's code hash and size should always be present_.

The next step is to decode the proof from the RLP block. Some information is displayed on the screen:

```
Verkle proof:
 * verification hints: 1 1 1 Present Present Present 
 * commitments: b8ce3d5e8857cec668d54ec13353ec237ba776832452a28514800efd75898366 49a58622affc8f8cc93042c7d5977127823d382e2602f3dc3e79e984b5dcd161 9fa9e535b55a646a53dbee71d0182a52457558187d00aba5945c3865fccd5130 4b084aefe7c95d371d42ba690a950e9144b78b1be51b037cf2c7282d8d015bd7 c0691f07dd31120f69b8d5d0b9adad2ef1da65b7f32e22fa8128980e43f1d23b d24543030c25a08399df898b9f3f8f1e0efb9c96ffa79402fdfaab7e9c522e8c 
```

The three `1` indicate that there are three "stems" and that they are at depth 1, and the following three `Present` indicate that these stems were present in the tree. Then, the commitments used in the proof are listed.

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

## TODO

 * [x] RLP encoding of the structure encapsulating the proof and the (key, values) tuples
 * [ ] SSZ encoding of that same structure when the final format has been defined
 * [ ] Add post state to the block
