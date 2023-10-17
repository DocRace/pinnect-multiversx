use multiversx_sdk::{
  ManagedAddress, ManagedBuffer, 
  token::{TokenIdentifier, TokenMint, TokenBurn},
  abi::{EndpointDefinition, TypeAbi}
};

use cyberconnect::get_social_influence;

#[derive(TopEncode, TopDecode, TypeAbi)]
struct Stake {
  pub voter: ManagedAddress,
  pub amount: BigUint,  
}

#[multiversx_ce::contract]
pub trait StakingAndVoting {

  #[init]
  fn init(&self) {
    self.token_id().set(TokenIdentifier::new(
      self.blockchain().get_sc_address(), 
      "VOTE"
   ));

  self.balances().clear();
}


  #[endpoint(mintTokens)]
  fn mint_tokens(&self, to: &ManagedAddress, amount: BigUint) -> SCResult<()> {

    let token_id = self.get_token_id();

    TokenMint {
      token: token_id,
      amount,
      to: to.clone() 
    }.call_and_fund_sc(self);

    Ok(())
  }

  #[endpoint(stakeTokens)]
  fn stake_tokens(&self, user: &ManagedAddress, proposal_id: u32) -> SCResult<()> {

    let amount = self.get_token_balance(user);
    
    let stake = Stake {
      voter: user.clone(),
      amount 
    };

    self.stakes(&proposal_id).push(stake);

    Ok(())
  }

  #[view(getTokenId)]
  fn get_token_id(&self) -> TokenIdentifier {
    self.token_id().get()  
  }

  #[view(getTokenBalance)]
  fn get_token_balance(&self, user: &ManagedAddress) -> BigUint {
    self.balances().get(user).result_or_default(BigUint::zero())
  }

  #[view(getStakes)]
  #[storage_mapper("stakes")]
  fn stakes(&self, proposal_id: &u32) -> StakeListMapper<Stake>;

  #[storage]
  trait StakingAndVotingStorage {
  
    #[view(getTokenId)]
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;
  
    #[view(getTokenBalance)]
    #[storage_mapper("balances")]
    fn balances(&self) -> MappingMapper<ManagedAddress, BigUint>;
    
  }  
}