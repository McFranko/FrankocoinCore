# Frankocoin Core
I'm trying to make a crypto focused on compressing the amount of data required for typical transactions.

The main part of Frankocoin is Frankolang, which is just a scripting language
The language is actually written as byte code, so some functions may not even have an actual utf8 code fort them
The reason for that is to keep everything as small as possible.
For example instead of writing "pay 'utxohash' (amount from utxoHash) to 'recievers public key/address'" You'd be writing:

0x1 0x(utxohash) 0x(publickey/address in der format)
Assuming that 0x1 is the code for paying a utxoHash to someone (I haven't come up with a definitive plan for all the instructions yet)

I've found that'll save around 30 bytes per transaction or something like that compared to my original plan.
where I had the instructions written as human understandable words. I've found a typical transaction with one input address and one output address will take up at most like 160 bytes. In the future I may make a more human readable version of the language that compiles down to the byte code

If you run a node you'll be recieving blocks from miners, verifying the work and code signatures, and then interpreting the code. The frankolang interpreter will write the various data to the files it needs to and so on.

One other big part of Frankocoin is the mining algorithm I intend on implementing. The idea is that miners will have to hash every line of code from the last n blocks (n is determined based on the amount of memory you want miners to use) and then add all those hashes together, add a nonce, hash it again and then determine if the resulting hash has a certain number of leading zeros to it. If it does, that work is valid.
This accomplishes two things: a) it has a high memory requirement making it asic resistant, and b) it forces miners to keep a copy of the blockchain (or a part of it).

Well there is no incentive for miners to host a server and run a full node, it does mean that they have to check the blockchain, and verify new blocks coming in. This means that it's much more likely for invalid blocks to be found and for the node who made them to be ousted. It contributes to a more secure and more verified network, making it safer for people who don't have the ability to run a node to be sure that the data they are getting is valid.

Thank you for coming to my ted talk.

Any help and suggestions are much appreciatted.
I have a notion page with a to do list and documentation, so use that if you feel like helping:
https://www.notion.so/Frankocoin-6e01c1e090014704b487b3b5f5df0025       Please add a comment on the corresponding page if you start working on something
Discord: McFronkle#0251
Email: mrelfranko@disroot.org
