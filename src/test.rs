#![cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};
    use soroban_sdk::token::Client as TokenClient;
    use soroban_sdk::token::StellarAssetClient;

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        env.mock_all_auths(); // Mocks the offline signature verification
        let admin = Address::generate(&env);
        let victim = Address::generate(&env);
        let merchant = Address::generate(&env);
        
        let token_admin = Address::generate(&env);
        let token_contract = env.register_stellar_asset_contract(token_admin.clone());
        let token = TokenClient::new(&env, &token_contract);
        let token_admin_client = StellarAssetClient::new(&env, &token_contract);

        let contract_id = env.register_contract(None, TulongTap);
        let contract = TulongTapClient::new(&env, &contract_id);

        // Fund the contract with NGO money
        token_admin_client.mint(&contract_id, &1000); 
        
        contract.init(&admin, &token_contract);
        contract.allocate_aid(&admin, &victim, &100);
        
        // Merchant submits the transaction the victim authorized offline
        contract.offline_pay(&victim, &merchant, &40);

        assert_eq!(token.balance(&merchant), 40);
    }

    #[test]
    #[should_panic(expected = "Not authorized admin")]
    fn test_unauthorized_allocate() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let attacker = Address::generate(&env);
        let victim = Address::generate(&env);
        let token_contract = Address::generate(&env);

        let contract_id = env.register_contract(None, TulongTap);
        let contract = TulongTapClient::new(&env, &contract_id);

        contract.init(&admin, &token_contract);
        contract.allocate_aid(&attacker, &victim, &100);
    }

    #[test]
    #[should_panic(expected = "Insufficient aid allowance")]
    fn test_insufficient_allowance() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let victim = Address::generate(&env);
        let merchant = Address::generate(&env);
        let token_contract = Address::generate(&env);

        let contract_id = env.register_contract(None, TulongTap);
        let contract = TulongTapClient::new(&env, &contract_id);

        contract.init(&admin, &token_contract);
        contract.allocate_aid(&admin, &victim, &20); // Only 20 allocated
        
        contract.offline_pay(&victim, &merchant, &50); // Trying to spend 50
    }

    #[test]
    fn test_state_verification() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let victim = Address::generate(&env);
        let token_contract = Address::generate(&env);

        let contract_id = env.register_contract(None, TulongTap);
        let contract = TulongTapClient::new(&env, &contract_id);

        contract.init(&admin, &token_contract);
        
        let key = DataKey::Allowance(victim.clone());
        let initial_allowance: i128 = env.as_contract(&contract_id, || env.storage().persistent().get(&key).unwrap_or(0));
        assert_eq!(initial_allowance, 0);

        contract.allocate_aid(&admin, &victim, &100);
        
        let updated_allowance: i128 = env.as_contract(&contract_id, || env.storage().persistent().get(&key).unwrap_or(0));
        assert_eq!(updated_allowance, 100);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized_spend() {
        let env = Env::default();
        // NOT mocking auths here to simulate a merchant trying to drain a victim's 
        // allowance without their offline signature.
        let admin = Address::generate(&env);
        let victim = Address::generate(&env);
        let merchant = Address::generate(&env);
        let token_contract = Address::generate(&env);

        let contract_id = env.register_contract(None, TulongTap);
        let contract = TulongTapClient::new(&env, &contract_id);

        // This should fail because `victim.require_auth()` in the contract 
        // won't find a valid signature for the victim.
        contract.offline_pay(&victim, &merchant, &10);
    }
}