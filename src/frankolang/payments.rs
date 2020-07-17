#[allow(non_snake_case)]

pub struct Payment {
    sender: [u8; 32],
    receiver: [u8; 32],
    signature: [u8; 64],
    valueSent: u64,
}

impl Payment {
    pub fn doesSenderHaveEnoughFunds(&self) -> bool {
        true
    }
}

pub static mut balances: Balances = Balances {
    vec: Vec::new()
};

pub struct Balances<'a>{
    vec: Vec<Balance<'a>>
}

impl Balances<'_> {
    pub fn asFileFormat(&self) {

    }

    pub fn from(buffer: &[u8]) {

    }

    pub fn indexPublicKey(&self, publicKey: &[u8]) -> Option<Balance> {
        match self.vec.iter().position(|&balance| balance.publicKey == publicKey) {
            Some(index) => Some(self.vec[index]),
            None => None
        }
    }
}

struct Balance<'a> {
    publicKey: &'a[u8],
    balance: u64,
    recievingTransactions: Vec<&'a[u8]>
}

impl Balance<'_> {
    pub fn addSignature(&self, payment: Payment) {
        self.recievingTransactions.push(&payment.signature);
        self.balance += payment.valueSent;
    }

}
