use multiversx_sdk::{
  ManagedAddress, ManagedBuffer,
  token::{TokenIdentifier, TokenMint, TokenBurn},
  abi::{EndpointDefinition, TypeAbi}  
};

use cyberconnect::{
  get_social_graph,
}; 

#[derive(TopEncode, TopDecode, TypeAbi)]
struct NFT {
  pub owner: ManagedAddress,
}

#[multiversx_ce::contract]
pub trait RevenueShare {

  #[init]
  fn init(&self) {
    self.next_token_id().set(&0u64);
  }

  #[endpoint(mintNFT)]
  fn mint_nft(&self, to: &ManagedAddress) -> SCResult<u64> {

    let token_id = self.get_next_token_id();
    
    let nft = NFT {
      owner: to.clone(),
    };

    self.nfts(&token_id).set(&nft);

    Ok(token_id)
  }

  #[endpoint(transferNFT)]
  fn transfer_nft(&self, 
                  sender: &ManagedAddress,
                  receiver: &ManagedAddress,
                  token_id: u64) -> SCResult<()> {
    
    let mut nft = self.nfts(&token_id).get();

    require!(nft.owner == *sender, "Not owner");

    nft.owner = receiver.clone();
    self.nfts(&token_id).set(&nft);  

    Ok(())
  }

  #[view(getNextTokenId)]
  fn get_next_token_id(&self) -> u64 {
    let current = self.next_token_id().get();
    self.next_token_id().set(&(current + 1));

    current
  }

  #[view(getNFT)]
  #[storage_mapper("nfts")]
  fn nfts(&self, token_id: &u64) -> NFTMapper<NFT>;

  #[storage]
  trait RevenueShareStorage {
    #[view(getNextTokenId)]
    #[storage_mapper("next_token_id")]
    fn next_token_id(&self) -> SingleValueMapper<u64>;
  }
}