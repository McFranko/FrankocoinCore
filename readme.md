# Frankocoin Core
Frankocoin is a cryptocurrency based around trying to compress the amount of
data needed for a typical transaction

Frankolang is the language that is used for interfacing with the database of
balances and sending transaction, and what not. Frankolang is designed to be as
small as possible so as to reduce transaction size. With frankolang,
a transaction with one "input" (as you would call it in bitcoin terminology;
frankocoin doesn't exactly have inputs and outputs) and one "output" and a fee,
it would take 148 bytes compare this to bitcoin, which for a one input, one
output transaction takes normally ~220 bytes.
A smaller transaction size means more transactions in a block, which means
lower fees

Frankohash is the mining algorithm used with frankolang. It requires that
miners keep a full (or almost full) copy of the blockchain. It does this by
having the miners hash every frankolang instruction and the header from each
block with a nonce, and then add all of that together into a single hash. That
hash must have a certain number of leading zeros. This makes it ASIC resistant
due to the high memory requirement. It also forces miners to keep a copy of the
blockchain. Well they don't have to make a full node (running a server where
others can access the blockchain), that does mean that there will be a lot more
people who have to check the blockchain and new blocks. This makes it more
secure

Thank you for coming to my ted talk

Any help and suggestions are much appreciatted.
I have a notion page with a to do list and documentation, so use that if you feel like helping:
https://www.notion.so/Frankocoin-6e01c1e090014704b487b3b5f5df0025       Please add a comment on the corresponding page if you start working on something

Discord: McFronkle#0251

Email: mrelfranko@disroot.org
