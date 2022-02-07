use std::time::SystemTime;


#[derive(Clone, Debug)]
pub struct Transaction
{
    sender: String,
    recipient: String,
    amount: f64,
    time: SystemTime,
}


impl Transaction
{
    pub fn new(sender: String, recipient: String, amount: f64, time: SystemTime) -> Self
    {
        Transaction {
            sender,
            recipient,
            amount,
            time,
        }
    }

    pub fn sender(&self) -> &str { &self.sender }

    pub fn recipient(&self) -> &str { &self.recipient }

    pub fn amount(&self) -> f64 { self.amount }

    pub fn time(&self) -> u128
    {
        self.time.duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis()
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        self.sender.as_bytes()
            .iter()
            .cloned()
            .chain(self.recipient().as_bytes()
                .iter()
                .cloned())
            .chain(self.amount().to_le_bytes()
                .iter()
                .cloned())
            .chain(self.time().to_le_bytes()
                .iter()
                .cloned())
            .collect()
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use std::ops::Add;


    #[test]
    fn test_as_bytes()
    {
        let transaction = Transaction {
            sender: String::from("bob"),
            recipient: String::from("alice"),
            amount: 1.0,
            time: SystemTime::UNIX_EPOCH.add(std::time::Duration::new(1234567890,0)),
        };

        let transaction_bytes = [
            98, 111, 98, 97, 108, 105, 99, 101, 0, 0, 0, 0, 0, 0, 240,
            63, 80, 4, 251, 113, 31, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        assert_eq!(transaction.as_bytes(), transaction_bytes);
    }
}
