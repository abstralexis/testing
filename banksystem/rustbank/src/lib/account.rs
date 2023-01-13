#[derive(PartialEq)]
pub struct Login {
    pub name: String,
    pub pin: u32
}

pub struct Vault {
    pub balance: f32
}

pub struct Account<'a> {
    pub account_number: u32,
    pub login: &'a Login,
    pub vault: Vault
}
impl<'a> Account<'a> {
    pub fn login(&self, login_submission: &Login) -> bool {
        login_submission == self.login
    }
}