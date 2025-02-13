# Mine a block - Vinteum challenge

The goal of this project is to remine the mythic block 170 containing the first bitcoin transaction ever from Satoshi to Hal Finney.

## Block specification

- Your block should include exactly two transactions: the coinbase transaction you will build yourself and the transaction whose hex serialization is available in the file `data/f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16.txt`
- Your block should be version 1.
- The previous block hash is `000000002a22cfee1f2c846adbd12b3e183d4f97683f85dad08a79780a84bd55`.
- The difficulty target should be `0xae77031e` (minimum difficulty in signet).
- Your block should present a valid merkle root.
- The block timestamp should not be earlier than 2009-01-12 01:22:03 UTC and no later than 2h in the future from the time you mine your block.

## Coinbase transaction specification

- Should be version 1.
- Should have exactly one input.
- The `scriptSig` should have between 2 and 100 bytes, you can put any information here. Miners use this space to identify themselves (try adding your github handle) and to increase the nonce search space (not required for CPU mining in low difficulty).
  - Optional: implement BIP34.
- Should have exactly one output. Since there will be no fees to collect, reward should be exactly 50 BTC.
- The `scriptPubkey` should be an anyone can spend script (`0x51`).

## How to check your implementation

You can recreate the historical block 170 using `timestamp = 0x496ab951 = Monday, 12 January 2009 03:30:25 GMT` and `difficulty = ffff001d` (minimum difficulty in mainnet) in your block header and the coinbase transaction data included in `b1fea52486ce0c62bb442b530a3f0132b826c74e473d1f2c220bfa78111c5082.txt`. Note that the minumum difficulty in mainnet is still quite challenging for CPU mining and might take dozens of minutes to compute in a modern machine if you are using a high level language such as Python.

This approach provides a path to validate your implementation because you can check your result against the blockchain data shown below (this may be particularly useful for checking your merkle root implementation and the difficulty targeting).

## Submission

Your program must create a text file named `solution/mined_block.txt` with exactly one line containing the hex serialization of your block (see the example below).

You are allowed to write your code in any of the following programming languages:
- Python
- C
- C++
- C#
- Rust
- Go

The reviewer will run your program via the bash script `solution/mine_block.sh` which MAY BE EDITED BY STUDENTS if you need to install additional dependencies (e.g. from `pip`) or change the compilation pipeline.

You MAY import a hashing library to access functions like sha256, but you MAY NOT use a Bitcoin-specific library to avoid implementing transaction and block serialization and the merkle root computation yourself.

## Example output

The following is the serialization of block 170 mined by Satoshi itself. It won't pass the `anyone_can_spend` test but it will have the required proof of work. You can use this to check your implementation, but don't try to work backwards from it since changing a bit here will invalidate the merkle root and the proof of work of the block (or maybe try it to get a feel for the security model of the protocol yourself). Beware of unintentional EOL characters at your solution.

``` shell
$ cat solution/block.txt
0100000055bd840a78798ad0da853f68974f3d183e2bd1db6a842c1feecf222a00000000ff104ccb05421ab93e63f8c3ce5c2c2e9dbb37de2764b3a3175c8166562cac7d51b96a49ffff001d283e9e700201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0102ffffffff0100f2052a01000000434104d46c4968bde02899d2aa0963367c7a6ce34eec332b32e42e5f3407e052d64ac625da6f0718e7b302140434bd725706957c092db53805b821a85b23a7ac61725bac000000000100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000
```
