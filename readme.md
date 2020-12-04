# Frankocoin
Frankocoin is a concept for a scalable and private cryptocurrency. It will be
able to process transactions at a rate 70%\* higher than bitcoin, and
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
transactions have a minimum size of around 170 bytes, but typically fall in
the 500-700 byte range. My way to achieve this is instead of storing the whole
transaction on the blockchain, only a hash of the transaction's
code, the public key, the signature, and hashes of the previous transactions
that gave you the coins you are spending. A single input, single output
transaction would take 128 bytes, and the code can be as complex and long as
you want without taking up any extra space on the blockchain. In order to make a
transaction you would do the following:

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
3.  I would send the transaction's script, my public key, the signature, and the
    hashes of the transactions I am spending to a node, and wait for it to be
    included in a block. I would also send the transaction script to the
    receiver.
4.  The receiver gets the transaction script, and makes a hash of it. If they
    see this hash show up on the blockchain, that means that it's validated and
    they can repeat the same process to spend their new coins.

On the node's side of this, they would be checking to make sure that all the
previous transaction hashes that I sent have not already been sent, and are
owned by my public key

This is somewhat analogous to writing a check. The transaction script is the
cheque. However, in order for the cheque to be valid, their needs to be a
compressed copy of it on the blockchain. The version that's on the blockchain
doesn't actually contain the data of the transaction, just a very compressed
version.

This also has the side effect of making Frankocoin much more private
than bitcoin. On bitcoin, you can find the balance of any address just by
looking at the blockchain. With this protocol however, there isn't any info
about the transactions displayed on the blockchain, just their hash. Only the
owners of the coins know their balances.

Discord: McFranko#0251

Email: mrelfranko@disroot.org

\* This is assuming both networks are running at their theoretical maximum,
meaning that all transactions are one input and one output, that all bitcoin
transactions are using segwit, and both networks create a block every ten
minutes. The actual block size on segwit can have a theoretical maximum of 4
megabytes, but that won't ever happen. Instead they tend to cap out at 2.5
megabytes.\
A one input one output transaction on bitcoin with segwit takes 190 bytes, and a
corresponding one on Frankocoin would take 128 bytes.
```
2621440 / 190 = 13797
2621440 / 128 = 20480

13797 / 20480 * 100 = 67.4%
```
One other notable thing is that in bitcoin, a scripted transaction takes up more
and more space the more complex the transaction gets. In Frankocoin, the
transaction code is always the same 16 bytes.
