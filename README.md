# Example of a block root with a Verkle state root

## Block content

This is a standard RLP block containing 3 transactions, and an added `VerkleProof` field at the end of the block header.

```
> rlpdump block2.rlp
[
  # Block Header
  [
    2a0fa77c9673bac5696974f6693b917bd391bcbfe564e1d246ebaa9835255c8a,
    1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347,
    0000000000000000000000000000000000000000,
    03aec4276033869b5900313c7d807cdaa948e38acd9aa43c98e05089adc7389d,
    145d1893587d0bcae212d39729190212236500e5c441a269f980381c611bebac,
    251fdcdd3e713f5799e22aaec5341cdc5874392a80e89b2f70766a83806f77d4,
    00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    020000,
    02,       # Block number
    47e7c4,
    018768,
    14,
    "",
    0000000000000000000000000000000000000000000000000000000000000000,
    0000000000000000,
    2de97bd1,
    
    # Serialized proof
00000000030000000a0a0a06000000be4a1372e5326571de085bea07f69308c2dc954f00a78b750832e806268ac8c619c836147396f92e35df8d9d0d963708597ea397bd9f988ec7207bdab425510075b694f796c26b56148eefd21002f82a12c2096742e7599330db4e7c1c323109c72d2c09d42931fa18c42e4cf905498b18f15885baf09ad097e8a9ad8ac875a0c09a4dfbc0861f29283511622fb9e8ccc46d9b08902b5a9ac9f28867e67a331ca0c5d50fa253408637b484d376f1606d5e35af73662665a3103af88b088fac9d676c268880a04bd5283e013eef51ece656da17454f1932006d134185a37c120006a24e627e0ca600835488a62a2d66db26bfe11963c0e2b4a808900e0a3f2cb4a04541c5bd5a1d11c5d6bdcc7d6638c999020f2a58fc089953e7f6476366071ebebbad5c9fc02936c9a811a230073de9e764d86bb4ef241559662f2605409b39def04a64a7bf574bf17220ad3493be6e1888096b56d9f48149636f8f25ca2b24cefef831e0fff121427fd895e4260c488c387f2e8c491912e4d82320f9257f581257bd7e52fbd5075945657c7f990b89bf83e820300e8b87ed891c843643c1d534c6c6ac250b313dd395e0fb47af3794eadaa0bb24c3bc00de7b34b1c1ecf2458de9ef6a782eaeb444193d097184ec52ecec8e1e19309e7b930cc861fec42bd027b4e075e3e04a57b98cd287d9d722959e738746f83a9df93519b37bf4c2ca02617476dc45023283cb93c9d4160838f608aac42513016ebd78b865bd74396f3966995cae42c92188f7419730fbed52b7c0577a0da0b337239c07b6c1d39af0b5418c88c1f4ba7d7cc59ff6f5e1f8b4d9e9e8d153b930f0cf391b0c4740d06862918663acb8a67a534a61339a634d80e82962f217b1ad8f0952f73c2a99672fbcd3c22c71d3f2ca61621d9b52010c7bc15a9a8f1714af0fdf7dca20200476f7b84aabdd080b81a1911b643ac8d9c5ed7f83f6d55a49765e304012ab97794c67293aacd2f4135b52ee43e677e425c0c49a90e68847f217eb6f450c1315cf094887003de25756e237c068d078a4fefb79890edcfc4a40f6f779868cf74af2831807,

    # List of (key, values) of the pre-state, that are accessed during the
    # block's execution.
    [
      [
        318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d01,
        320122e8584be00d,
      ],
      [
        e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526400,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d03,
        "",
      ],
      [
        18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a02,
        0000000000000000,
      ],
      [
        318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d02,
        0300000000000000,
      ],
      [
        18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a04,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526402,
        0000000000000000,
      ],
      [
        e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526403,
        c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
      ],
      [
        318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d04,
        "",
      ],
      [
        18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a00,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a03,
        c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
      ],
      [
        e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526401,
        1bc176f2790c91e6,
      ],
      [
        e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526404,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d00,
        0000000000000000000000000000000000000000000000000000000000000000,
      ],
      [
        18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a01,
        e703,
      ],
    ],
  ],
  
  # Transaction list
  [
    [
      03,
      342770c0,
      878c,
      0102030000000000000000000000000000000000,
      03e7,
      "",
      "&",
      b5052005e25c9c5f8100a37d9aad8c3057969170cb239b03b7615c5619b73979,
      50d23a8e2796503706568c13de56d45a232903b84f55a5c663383988fb559642,
    ],
    [
      04,
      342770c0,
      878c,
      0000000000000000000000000000000000000000,
      03e7,
      "",
      "%",
      f8ea4ce9a209cf2feee581a3a5e263bd48d0732c77ae9c723f7ec0a18ac16dd8,
      6f0e4bd905d8acbb6273fd6bed78548e9544bf48634d86234715b4f92bf89bfe,
    ],
    [
      05,
      342770c0,
      "xP",
      0000000000000000000000000000000000000000,
      "",
      "",
      "%",
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

The block that is provided, can be decyphered with the program provided in this repository. At the moment, the proofs don't check, so this section will be limited to the parts that do, and gradually expanded as fixes are rolled in.

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

These are the same addresses that are present in the RLP dump above.

The next step is to decode the proof from the RLP block.

The final step is to verify the proof.

## Checking the proof with the associated test

The rust code also includes a unit test `compare_with_geth`, that rebuilds the tree as it exists in geth, and generates the proof from the rust code. It is possible to print out the expected proof in hex format by typing:

```
> cargo test -- --nocapture
(...)
running 1 test
serialized proof=00000000030000000a0a0a0600000053acaf9df78f9714054fdaaa630d6a3e60521c57d301411ac4c49668df25e40c49a58622affc8f8cc93042c7d5977127823d382e2602f3dc3e79e984b5dcd1614e221a9dfb42671da7ad1018131752c994bcd0c7b01441777cc7e09f646abc06b309948924dfefdfc44792e4c9fce1cd86c9145e16970d6d20bf61139d7a5ee48dec49d7e5c2ec27f35dc664ca337c8d01bfe276b8eb4fbc24910a557cf2cd30e386a2666ccf5191dc9caf569f183375444b6f48ebcff55330dd63bcdfdee8243e16c371b13a368de5ab953f44ad91ccb3a11568ea70e1bd64a36c6f5792ff0167cef8b0cf5dcc222f9b97f86f7bfa012d9fa917ce34a505bb1126ff9e064a45d2ce33a4b04145c0ad64c33f2bbf62558c44679d734413e92f9431bb4f7bb49f2d22da9e9640ab2e0cce48f1430096b77343842ae4688e8c785715b7da12e15529673b6b232e861b49301908e476ee009ac3acf34ef9d97ef75e4bcd9ac2a7ef40d53c09abae5e2e302e22f3e66b7cd7e1f7d8adaacec845975be4b40fbe041b2aa44e8c5aff2e363c3cc24d76e4cb8f20a815bbd587b3b59ff1bac5d0053f8fe34bc4fbfa222420f5b34cd7986a01c1da8273a289ecd8d3e393a1ea7af40f71bd514394cef0a1af38cfcefcb7fe3aa78ad980ad56c657b9e1945cb2b5fd3d04830270ebf638a707c3be1efff67dc1a5b8251ae9a540d403f55dba146d9062ae6e70e639f9ee2bd7a7394954466277dd9c1ef256df31497892de34e8de0d788cf525e47669e8a959e5c20bb62583937b99c2a4b95dd37be1ce24dbf9a18eceaf370ce0da389cccd432ad46442e7c18c33f13e5e2d736a284bcbddca3cf85a2dd4bf049d89f950c0f1e03ef19f9729425e4ef15cb06dbe0a54fbee7e135e89892d90d06856951d4f4f9d5fe1ea41a4bc249e9fca9a7eb4c2a55ed4a427109c4807062af2fb0c8c10141cf1bd798181039285cf3ab88c13401d35404bb4a80541f1d200a3eb8264a060e58f56084b818f034fdda222193bdbc10b0e5eb6f92edd563cb87a9da7587f9625e4c9a06757f2a2ba13613121b8b693304c05dc2f0f80a
```

One can see that the proof hints are identical:
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
