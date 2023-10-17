use multiversx_sdk::{
    ManagedAddress, ManagedBuffer, ESDTIdentifier,
    token::TokenIdentifier,
    abi::{EndpointDefinition, TypeAbi},
};

use cyberconnect::{
    get_social_graph, 
    get_user_info,
};

#[derive(TypeAbi)]
pub struct Creathon {
  pub owner: ManagedAddress,
}

#[multiversx_ce::contract]
pub trait CreathonContract {

  #[init]
  fn init(&self, owner: &ManagedAddress) {
    let creathon = Creathon {
        owner: owner.clone(),
    };  

    self.creathon().set(&creathon);
  }

  #[endpoint(submitWork)]
  fn submit_work(&self, user: &ManagedAddress) -> SCResult<()> {
    let caller = get_user_info(user)?;
    let is_admin = get_social_graph(self.creathon().get().owner, caller)
        .relationship == RelationshipType::Owner;

    if !is_admin {
        return sc_error!(Unauthorized);
    }  

    // Submit work logic
    let submissions = self.submissions(&user).get();
    submissions.push(work_data); 
    self.submissions(&user).set(&submissions);

    let total_submissions = self.total_submissions().get();
    self.total_submissions().set(&(total_submissions + 1));


    Ok(())
  }

  #[endpoint(castVote)]
  fn cast_vote(&self, voter: &ManagedAddress, proposal_id: u32, vote: bool) -> SCResult<()> {

    let vote = Vote {
      voter: voter.clone(),
      proposal_id,
      vote,  
    };

    self.votes(&proposal_id).update(|votes| {
      votes.push(vote);
      votes
    });

    Ok(())
  }

  #[view(getVotes)]
  #[storage_mapper("votes")]
  fn votes(&self, proposal_id: &u32) -> VoteSetMapper<Vote>;


  #[endpoint(distributeRewards)]
  fn distribute_rewards(&self, user: &ManagedAddress) -> SCResult<()> {

    // Distribute rewards logic
    let mut rewards = self.rewards(&user).get();
    rewards += reward_amount;
    self.rewards(&user).set(&rewards);
    
    let mut total_rewards = self.total_rewards().get();  
    total_rewards += reward_amount;
    self.total_rewards().set(&total_rewards);
    
    Ok(())
  }

  

}

#[multiversx_ce::storage]
trait CreathonStorage {

  #[view(getCreathon)]
  #[storage_mapper("creathon")]
  fn creathon(&self) -> SingleValueMapper<Creathon>;

  #[view(getSubmissions)]
  #[storage_mapper("submissions")]
  fn submissions(&self, user: &ManagedAddress) -> SetMapper<WorkData>;

  #[view(getTotalSubmissions)]
  #[storage_mapper("total_submissions")]
  fn total_submissions(&self) -> SingleValueMapper<u64>;

  #[view(getRewards)]  
  #[storage_mapper("rewards")]
  fn rewards(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

  #[view(getTotalRewards)]
  #[storage_mapper("total_rewards")]
  fn total_rewards(&self) -> SingleValueMapper<BigUint>;

}
