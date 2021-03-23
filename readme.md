This is now archived, this has moved to a new project,
[PADLOCK](https://github.com/mcfranko/padlock-whitepaper)

# Frankocoin
Frankocoin is a concept for a scalable and private cryptocurrency. It will be
able to process transactions at a rate 70%\* higher than bitcoin.

Most major cryptocurrencies suffer from the issue of
scalability. The standard networks consists of thousands of nodes, with each node
storing a copy of the blockchain which contains the entire history of
transactions. Fundamental to how cryptocurrencies work, is that
anyone can get a history of all the transactions and verify it themselves, which
provides an incredibly secure network; where transaction fraud is approaching impossible. The issue of course is that this is requires a lot of space to run
well. 

In order to counteract this, a single block has a fixed size limit. Currently, a
single bitcoin block can be no larger than 1 MiB (except segwit). This makes sure that the blockchain doesn't get too big, too quick. The
major issue there, is that there is a limit of transactions that can be stored in a
single block, thus slowing down the network. Bigger blocks make
the network faster, but less scalable, while smaller blocks make the network more
scaleable, yet slower.

Because of the given reasons, the solution is easy to identify: transactions need to take up less space. Bitcoin
transactions have a minimum size of around 180 bytes, but typically fall in
the 500-700 byte range. The way Frankocoin aims to change this is by not storing the whole
transaction on the blockchain. Only a hash of the transaction's
script, the public key, the signature, and hashes of the previous transactions,
that gave you the coins you are spending, are stored in the blockchain. A single input and single output transaction would take 128 bytes. Although the script can be as complex and long as
you want without taking up any extra space on the blockchain. In order to make a
transaction you would do the following:

1.  Write the script that would execute the transaction.
2.  Verify that you own enough coins to make the desired transaction by showing
    that there is a hash of a transaction that gave you the required amount of coins on the
    blockchain.
    -   Here is an example of the script:\
            pay 12 to ad649e8f6fh from a8969ddfcea99 signature=967e8fca72v\
            previousTransaction = {\
                pay 12 to a8969ddfcea99 from as746af878483 signature=8a67e6f6c4a2\
            }\
    -   The hash of this transaction would be:\
            004718a42c7c663804e2c5779927e591
    -   The previous transaction is to verify that you are not going into debt by overspending. The receiver should be able to make a hash of the
        previous transaction and then check if it is on the blockchain. If it has been recorded,
        that means coins have been received, and that someone is in posession of them.
3.  Your public key, the signature, and the
    hashes of the transactions  sent to a node, and wait for it to be
    processed and incorporated into the block. The transaction script would also be sent to the
    receiver.
4.  The receiver posesses transaction script, and makes a hash out of it. If a hash show up on the blockchain, that means that it's validated and
    they can repeat the same process with their coins.

On the node's side of this, they would be checking to make sure that all the
previous transaction hashes that were sent have not already been sent, and are
owned by the public key.

This is somewhat analogous to writing a check. The transaction script is the
cheque. However, in order for the cheque to be valid, their needs to be a
hashed copy of it on the blockchain. The version that's recorded on the blockchain
doesn't actually contain the data of the transaction, just a hashed
version which can be used to verify future transactions.

This also results in making Frankocoin much more private
than bitcoin. On bitcoin, you can find the balance of any address just by
looking at the blockchain. With Frankocoin's protocol however, there isn't any info
about the transactions displayed on the blockchain, just their hash. Only the
owners of the coins know their balances.

Discord: McFranko#0251

Email: mrelfranko@disroot.org

\* This is assuming both networks are running at their theoretical maximum,
meaning that all transactions are one input and one output, that all bitcoin
transactions are using segwit, and both networks create a block every ten
minutes. The actual block size on segwit can have a theoretical maximum of 4
megabytes, but that doesn't happen under normal circumstances. Instead they tend to cap out at 2.5
megabytes.\
A one input, one output transaction on bitcoin with segwit takes 190 bytes, and a
corresponding one on Frankocoin takes 128 bytes.
```
2621440 / 190 = 13797
2621440 / 128 = 20480

13797 / 20480 * 100 = 67.4%
```
One other notable thing is that in bitcoin, a scripted transaction takes up more
and more space the more complex the transaction gets. In Frankocoin, the
transaction script is consistently 16 bytes.
