use soroban_sdk::{
    contract, 
    contractimpl, 
    Address, 
    Env, 
    String, 
    symbol_short,
    token,
};
// use soroban_token_sdk::Token;

#[contract]
pub struct StellarAsset;

#[contractimpl]
impl StellarAsset {
    /// Initialize the token with metadata
    /// 
    /// # Arguments
    /// 
    /// * `env` - The contract environment
    /// * `admin` - The administrative address with minting privileges
    /// * `name` - Token name
    /// * `symbol` - Token symbol (ticker)
    /// * `decimals` - Number of decimal places
    // pub fn initialize(
    //     env: Env, 
    //     admin: Address, 
    //     name: String, 
    //     symbol: String, 
    //     decimals: u32
    // ) {
    //     // Create token client
    //     let token = token::Client::new(&env, &env.current_contract_address());
        
    //     // Set token metadata
    //     token.set_metadata(
    //         symbol_short!("ROCK"),  // Short symbol (max 4 chars)
    //         name,                   // Full name
    //         decimals                // Number of decimals
    //     );
    // }

    /// Mint new tokens to a specified account
    /// 
    /// # Arguments
    /// 
    /// * `env` - The contract environment
    /// * `admin` - Admin address
    /// * `to` - Address to receive minted tokens
    /// * `amount` - Number of tokens to mint
    pub fn mint(
        env: Env, 
        admin: Address, 
        to: Address, 
        amount: i128
    ) {
        // Verify caller is authorized
        admin.require_auth();

        let token = token::StellarAssetClient::new(&env, &env.current_contract_address());

        // Mint tokens
        token.mint(&to, &amount);
    }

    /// Transfer tokens between accounts
    /// 
    /// # Arguments
    /// 
    /// * `env` - The contract environment
    /// * `from` - Source account
    /// * `to` - Destination account
    /// * `amount` - Number of tokens to transfer
    pub fn transfer(
        env: Env, 
        from: Address, 
        to: Address, 
        amount: i128
    ) {
        // Ensure sender has authorized the transaction
        from.require_auth();

        let token_client = token::TokenClient::new(&env, &env.current_contract_address());
        token_client.transfer(&from, &to, &amount);
    }

    /// Burn tokens from an account
    /// 
    /// # Arguments
    /// 
    /// * `env` - The contract environment
    /// * `from` - Account to burn tokens from
    /// * `amount` - Number of tokens to burn
    pub fn burn(
        env: Env, 
        from: Address, 
        amount: i128
    ) {
        // Ensure account is authorized to burn
        from.require_auth();

        let token_client = token::TokenClient::new(&env, &env.current_contract_address());
        token_client.burn(&from, &amount);
    }

    /// Get token balance for an account
    /// 
    /// # Arguments
    /// 
    /// * `env` - The contract environment
    /// * `account` - Account to check balance
    /// 
    /// # Returns
    /// 
    /// Balance of the specified account
    pub fn balance(
        env: Env, 
        account: Address
    ) -> i128 {
        let token_client = token::TokenClient::new(&env, &env.current_contract_address());
        token_client.balance(&account)
    }
}
