use multiversx_sdk::{
    ManagedAddress, ManagedBuffer,
    token::{TokenIdentifier, Token Mint, Token Burn},
    abi::{EndpointDefinition, TypeAbi}
  };  
  
  use cyberconnect::get_connections;
  
  #[multiversx_ce::contract]
  pub trait DeployContract {
  
    #[init]
    fn init(&self) {
      self.token_id().set(TokenIdentifier::new(
        self.blockchain().get_sc_address(), 
        "PNNT"
      ));
    }
  
    #[endpoint(mintTokens)]
    fn mint_tokens(&self, to: &ManagedAddress, amount: BigUint) -> SCResult<()> {
  
      let token_id = self.get_token_id(); 
  
      TokenMint {
        token: token_id,
        amount,
        to: to.clone(),
      }.call_and_Fund_sc(self);
  
      Ok(())
    }
  
    #[endpoint(burnTokens)]
    fn burn_tokens(&self, from: &ManagedAddress, amount: BigUint) -> SCResult<()> {
  
      let token_id = self.get_token_id();
  
      TokenBurn {
        token: token_id,
        amount,
        from: from.clone(),
      }.call_and_Fund_sc(self); 
  
      Ok(())
    }
  
    #[view(getTokenId)]  
    fn get_token_id(&self) -> TokenIdentifier {
      self.token_id().get()
    }
    
    #[endpoint(getTokenSupply)]
    fn get_token_supply(&self) -> SCResult<BigUint> {
      let supply = self.supply().get();
      Ok(supply)
    }
      
    #[storage]
    trait DeployStorage {
      #[view(getTokenId)]
      #[storage_mapper("token_id")]
      fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;
    
      #[view(getTokenSupply)]
      #[storage_mapper("supply")]
      fn supply(&self) -> SingleValueMapper<BigUint>;
    }
}