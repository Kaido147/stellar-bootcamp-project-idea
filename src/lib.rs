#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token};

#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Allowance(Address) // Maps a Victim's Address to their remaining USDC allowance
}

#[contract]
pub struct TulongTap;

#[contractimpl]
impl TulongTap {
    /// Initializes the relief vault with an NGO Admin and the USDC token address
    pub fn init(env: Env, admin: Address, token: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    /// Admin allocates a specific amount of aid to a victim's address
    pub fn allocate_aid(env: Env, admin: Address, victim: Address, amount: i128) {
        admin.require_auth();
        let current_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != current_admin { panic!("Not authorized admin"); }

        let key = DataKey::Allowance(victim.clone());
        let current_allowance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        
        env.storage().persistent().set(&key, &(current_allowance + amount));
    }

    /// Process an offline payment. The Victim signs the payload off-chain.
    /// The Merchant submits this transaction online later.
    pub fn offline_pay(env: Env, victim: Address, merchant: Address, amount: i128) {
        // THE MAGIC: Soroban's host environment natively verifies the victim's 
        // off-chain signature attached to this transaction call.
        victim.require_auth();

        let key = DataKey::Allowance(victim.clone());
        let current_allowance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        
        if current_allowance < amount {
            panic!("Insufficient aid allowance");
        }

        // Deduct the spent amount from the victim's virtual allowance
        env.storage().persistent().set(&key, &(current_allowance - amount));

        // Transfer actual USDC from the Contract Vault directly to the Merchant
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &merchant, &amount);
    }
}