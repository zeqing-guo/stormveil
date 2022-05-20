# stormveil

```sh
cp .env.example .env
cp address.example address

# setup .env
vim .env

# add addresses line by line
vim address 

cargo build --release
./run.sh
```

Output:

``` sh
➜  stormveil git:(main) cargo build --release && ./run.sh
    Finished release [optimized] target(s) in 0.07s
address: 0x6c2c06790b3e3e3c38e12ee22f8183b37a13ee55
num of txs: 10000

address: 0x32Eb7902D4134bf98A28b963D26de779AF92A212
num of txs: 10000
0x32Eb7902D4134bf98A28b963D26de779AF92A212, 0x095ea7b3, approve, 0x80f544d5c63f7f1f1890fb8ac2be3e8c716ca771b87951c8b0c53a36b035df2a
0x32Eb7902D4134bf98A28b963D26de779AF92A212, 0xa9059cbb, transfer, 0x0850c2a559cd13a2cc6e91095c2d4d7e09326f8cecbc536ab49a909bd53369e0

address: 0x0C1Cf6883efA1B496B01f654E247B9b419873054
num of txs: 6472
0x0C1Cf6883efA1B496B01f654E247B9b419873054, 0x022c0d9f, swap, 0x9dbc9ddb95ad100ed7b8e88031921a30a26866226b95f5f1fa84b7ef6f5e00e6
0x0C1Cf6883efA1B496B01f654E247B9b419873054, 0x095ea7b3, approve, 0xb9bc81103bd2573d5879f0926de89434f9be4496131a4e2104916f8cad3f0c78
0x0C1Cf6883efA1B496B01f654E247B9b419873054, 0xa9059cbb, transfer, 0xb9ad1b52f1fbe9a60b3f932fb3cf14cd7faf8c274ba2932ce3e621a60f867e00
```
