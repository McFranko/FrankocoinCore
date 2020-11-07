# Frankocoin

Frankocoin is a concept for a scalable and private cryptocurrency. It will be
able to process transactions at a rate ten times higher than bitcoin, and
provide much more privacy in the process.

Almost all cryptocurrencies (especially bitcoin) suffer from the issue of
scalability. The bitcoin network consists of thousands of nodes. Each node
stores a copy of the blockchain which contains the entire history of
transactions. This part is fundamental to how cryptocurrencies work, is that
anyone can get a history of all the transactions and verify it themselves, which
thus provides an incredibly secure network where transaction fraud is next to
impossible. The issue of course is that this is requires a lot of space to run
well. 

In order to counteract this, a single block (a group of transactions,
which is generated every ten minutes) has a fixed size limit. Currently, a
single bitcoin block can be no larger than 1 MiB (although kind of 4 if using
segwit). This makes sure that the blockchain doesn't get too big too quick. The
issue there, is that it limits the amount of transactions that can be in a
single block, thus slowing down the network. So essentially, bigger blocks make
the network faster, but less scalable, well smaller blocks make the network more
saleable but slower.

The solution is obvious: transactions need to take up less space. Bitcoin
transactions have a minimum size of around 200-250 bytes, but typically fall in
the 500-700 byte range. My way to achieve this is instead of storing the whole
transaction on the blockchain, only a 16 byte hash of the transaction is
stored. So in order to make a transaction I would do the following:

1.  Write the script that would execute the transaction
2.  Prove that I own enough coins to make the desired transaction by showing
    that there is a hash of a transaction that gave me coins on the
    blockchain
    -   Here is an example of the script:\
            pay 12 to ad649e8f6fh from a8969ddfcea99 signature=967e8fca72v\
            previousTransaction = {\
                pay 12 to a8969ddfcea99 from as746af878483 signature=8a67e6f6c4a2\
            }\
    -   The hash of this transaction would be:\
            004718a42c7c663804e2c5779927e591
    -   The previousTransaction is to prove that I actually own coins. If I own
        coins, the receiver should be able to make a hash of the
        previousTransaction and then check if it is on the blockchain. If it it,
        that means someone sent me coins, and that I own them.
3.  I would send the new transaction hash to a node, and wait for the
    transaction hash to be stored on a block.
4.  The receiver of my transaction is now free to repeat the same process as me
    and spend their coins
This is somewhat analogous to writing a check. The transaction script is the
cheque. However, in order for the cheque to be valid, their needs to be a
compressed copy of it on the blockchain. The version that's on the blockchain
doesn't actually contain the data of the transaction, just a very compressed
version. This also has the side effect of making Frankocoin much more private
than bitcoin. On bitcoin, you can find the balance of any address just by
looking at the blockchain. With this protocol however, there isn't any info
about the transactions displayed on the blockchain, just their hash.

Discord: McFranko#0251

Email: mrelfranko@disroot.org
