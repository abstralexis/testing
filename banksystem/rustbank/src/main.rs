use lib::account::*;

fn main() {
    let login: Login = Login {name: "Bruh".to_owned(), pin: 10100};

    let account = Account {
        account_number: 329847,
        login: &login,
        vault: Vault { balance: 420.69 }
    };
    println!("Is login valid: {:?}", account.login(&login));
}
