use multiversx_sdk::{
    ManagedAddress, ManagedBuffer,
    token::{TokenIdentifier, TokenMint, TokenBurn}, 
    abi::{EndpointDefinition, TypeAbi}
};

use cyberconnect::get_social_graph;

#[derive(TopEncode, TopDecode, TypeAbi)]
struct Location {
    pub owner: ManagedAddress,
    pub name: String,
    pub description: String,
}

#[multiversx_ce::contract]
pub trait MagiPopMap {

    #[init]
    fn init(&self) {
        self.next_id().set(&0u64);
    }

    #[endpoint(createLocation)]
    fn create_location(&self, 
                    owner: &ManagedAddress,
                    name: String,
                    description: String) -> SCResult<u64> {
    
    let id = self.get_next_id();
    
    let location = Location {
        owner: owner.clone(),
        name,
        description,
    };

    self.locations(&id).set(&location);

    Ok(id)
    }

    #[endpoint(updateLocation)]
    fn update_location(&self, 
                    id: u64,
                    description: String) -> SCResult<()> {

    let mut location = self.locations(&id).get();
    location.description = description;  
    self.locations(&id).set(&location);

    Ok(())
    }

    #[view(getNextId)]
    fn get_next_id(&self) -> u64 {
        let current = self.next_id().get();
        self.next_id().set(&(current + 1));
        current 
    }

    #[view(getLocation)]
    #[storage_mapper("locations")]
    fn locations(&self, id: &u64) -> LocationMapper<Location>;

    #[storage] 
    trait MagiPopMapStorage {
        #[view(getNextId)]
        #[storage_mapper("next_id")]
        fn next_id(&self) -> SingleValueMapper<u64>;
    }

}