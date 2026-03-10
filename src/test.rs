#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{Env, Address, String, testutils::Address as _};

#[test]
fn test_token_creation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenStudio);
    let client = TokenStudioClient::new(&env, &contract_id);
    
    // Mock authentication
    env.mock_all_auths();
    
    // Create a token
    let creator = env.current_contract_address();
    
    let token_name = String::from_str(&env, "My Test Token");
    let token_symbol = String::from_str(&env, "MTT");
    let initial_supply = 1000_0000000i128;
    
    let token_id = client.create_token(
        &token_name,
        &token_symbol,
        &initial_supply,
        &None,
    );
    
    assert_eq!(token_id, 1);
    
    // Verify token info
    let info = client.get_token_info(&token_id).unwrap();
    assert_eq!(info.name, token_name);
    assert_eq!(info.symbol, token_symbol);
    assert_eq!(info.owner, creator);
    assert_eq!(info.contract_address, creator); // For now, address is same as creator
    
    // Check total tokens
    assert_eq!(client.total_tokens(), 1);
}

#[test]
fn test_multiple_tokens() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenStudio);
    let client = TokenStudioClient::new(&env, &contract_id);
    
    env.mock_all_auths();
    
    let creator = env.current_contract_address();
    
    // Create 3 tokens
    for i in 0..3 {
        let token_name = String::from_str(&env, &format!("Token {}", i));
        let token_symbol = String::from_str(&env, &format!("TK{}", i));
        
        let token_id = client.create_token(
            &token_name,
            &token_symbol,
            &0i128,
            &None,
        );
        
        assert_eq!(token_id, i + 1);
    }
    
    assert_eq!(client.total_tokens(), 3);
    
    // Query tokens by owner
    let tokens = client.get_tokens_by_owner(&creator, &0, &10);
    assert_eq!(tokens.len(), 3);
}

#[test]
fn test_token_address_retrieval() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenStudio);
    let client = TokenStudioClient::new(&env, &contract_id);
    
    env.mock_all_auths();
    
    let token_id = client.create_token(
        &String::from_str(&env, "Test"),
        &String::from_str(&env, "TST"),
        &0i128,
        &None,
    );
    
    // Token address should exist
    let addr = client.get_token_address(&token_id);
    assert!(addr.is_some());
}

#[test]
fn test_pagination() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenStudio);
    let client = TokenStudioClient::new(&env, &contract_id);
    
    env.mock_all_auths();
    
    let creator = env.current_contract_address();
    
    // Create 5 tokens
    for i in 0..5 {
        let token_name = String::from_str(&env, &format!("Token {}", i));
        let token_symbol = String::from_str(&env, &format!("TK{}", i));
        
        client.create_token(
            &token_name,
            &token_symbol,
            &0i128,
            &None,
        );
    }
    
    // Test pagination
    let page1 = client.get_tokens_by_owner(&creator, &0, &2);
    assert_eq!(page1.len(), 2);
    
    let page2 = client.get_tokens_by_owner(&creator, &2, &2);
    assert_eq!(page2.len(), 2);
    
    let page3 = client.get_tokens_by_owner(&creator, &4, &2);
    assert_eq!(page3.len(), 1);
}

#[test]
fn test_empty_owner_query() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenStudio);
    let client = TokenStudioClient::new(&env, &contract_id);
    
    env.mock_all_auths();
    
    let other_user = Address::generate(&env);
    
    // Query tokens for user with no tokens
    let tokens = client.get_tokens_by_owner(&other_user, &0, &10);
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_zero_total_tokens() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenStudio);
    let client = TokenStudioClient::new(&env, &contract_id);
    
    // Initially total tokens should be 0
    assert_eq!(client.total_tokens(), 0);
}