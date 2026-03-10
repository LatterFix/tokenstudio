#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String, Vec,
    symbol_short,
};

// ========== Factory Contract ==========

// Storage keys for the factory
#[contracttype]
pub enum FactoryDataKey {
    TotalTokens,
    TokenOwner(u32),
    TokenAddress(u32),
    TokenInfo(u32),
}

// Metadata for a created token
#[contracttype]
#[derive(Clone, Debug)]
pub struct TokenInfo {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub owner: Address,
    pub contract_address: Address,
    pub created_at: u64,
}

#[contract]
pub struct TokenStudio;

#[contractimpl]
impl TokenStudio {
    /// Initialize the factory - called once at deployment
    pub fn __constructor(env: Env) {
        // Set initial token count to 0
        env.storage().instance().set(&FactoryDataKey::TotalTokens, &0u32);
    }

    /// Create a new token
    /// 
    /// # Arguments
    /// * `name` - The full name of the token (e.g., "My Token")
    /// * `symbol` - The token symbol (e.g., "MTK")
    /// * `_initial_supply` - Initial tokens to mint to the owner (reserved for future use)
    /// * `_admin` - Optional custom admin (if None, creator is admin) (reserved for future use)
    /// 
    /// # Returns
    /// The ID of the newly created token
    pub fn create_token(
        env: Env,
        name: String,
        symbol: String,
        _initial_supply: i128,
        _admin: Option<Address>,
    ) -> u32 {
        let creator = env.current_contract_address();
        
        // Get the next token ID
        let mut token_id: u32 = env.storage()
            .instance()
            .get(&FactoryDataKey::TotalTokens)
            .unwrap_or(0);
        
        token_id += 1;
        
        // For now, just use the creator address as a placeholder for the token contract
        // In production, you would deploy an actual token contract and get its address
        let token_address = creator.clone();
        
        // Store token metadata
        let token_info = TokenInfo {
            id: token_id,
            name: name.clone(),
            symbol: symbol.clone(),
            owner: creator.clone(),
            contract_address: token_address.clone(),
            created_at: env.ledger().timestamp(),
        };
        
        env.storage().instance().set(&FactoryDataKey::TokenAddress(token_id), &token_address);
        env.storage().instance().set(&FactoryDataKey::TokenOwner(token_id), &creator);
        env.storage().instance().set(&FactoryDataKey::TokenInfo(token_id), &token_info);
        env.storage().instance().set(&FactoryDataKey::TotalTokens, &token_id);
        
        // Emit event (using short symbol)
        env.events().publish(
            (symbol_short!("token_crt"), token_id),
            (creator, token_address, symbol)
        );
        
        token_id
    }

    /// Get information about a created token
    pub fn get_token_info(env: Env, token_id: u32) -> Option<TokenInfo> {
        env.storage().instance().get(&FactoryDataKey::TokenInfo(token_id))
    }

    /// Get the contract address of a created token
    pub fn get_token_address(env: Env, token_id: u32) -> Option<Address> {
        env.storage().instance().get(&FactoryDataKey::TokenAddress(token_id))
    }

    /// Get total number of tokens created
    pub fn total_tokens(env: Env) -> u32 {
        env.storage().instance().get(&FactoryDataKey::TotalTokens).unwrap_or(0)
    }

    /// Get all tokens created by a specific address
    pub fn get_tokens_by_owner(env: Env, owner: Address, start: u32, limit: u32) -> Vec<TokenInfo> {
        let mut result = Vec::new(&env);
        let total = Self::total_tokens(env.clone());
        
        let end = (start + limit).min(total);
        
        for i in start..end {
            let token_id = i + 1; // Token IDs start at 1
            if let Some(token_info) = Self::get_token_info(env.clone(), token_id) {
                if token_info.owner == owner {
                    result.push_back(token_info);
                }
            }
        }
        
        result
    }
}