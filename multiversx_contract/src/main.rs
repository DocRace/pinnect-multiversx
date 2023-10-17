use multiversx_sdk::{
    *,
    abi::{EndpointDefinition},
    Vec, String,
};

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct Map {
    pub name: String,
    pub description: String,
    pub image: String, 
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct Location {
    pub owner: ManagedAddress,
    pub map_id: ManagedAddress,
    pub name: String,
    pub x: BigUint,
    pub y: BigUint,
    pub tags: String,
    pub image: String,
    pub is_on_chain: bool,
}

#[multiversx_ce::contract]
pub trait MagiPopMap {

    #[endpoint(createMap)]
    fn create_map(&self, name: String, description: String, image: String) -> SCResult<()> {
        // Store map in storage
        let map = Map {
            name,
            description,
            image,
        };
        self.map().set(&map);
        Ok(())
    }

    #[endpoint(createLocation)]
    fn create_location(
        &self,
        owner: &ManagedAddress,
        map_id: &ManagedAddress,
        name: String,
        x: BigUint,
        y: BigUint,
        tags: String,
        image: String,
    ) -> SCResult<()> {
        let location = Location {
            owner: owner.clone(),
            map_id: map_id.clone(),
            name,
            x,
            y,
            tags,
            image,
            is_on_chain: true, 
        };
        
        // Store location in storage
        self.locations(&location.owner).update(|locations| {
            locations.push(location);
            locations
        });
        
        Ok(())
    }
}

#[multiversx_ce::storage]
pub trait MagiPopMapStorage {
    #[view(getMap)]
    #[storage_mapper("map")]
    fn map(&self) -> SingleValueMapper<Map>;

    #[view(getLocations)]
    #[storage_mapper("locations")] 
    fn locations(&self, owner: &ManagedAddress) -> LinkedListMapper<Location>;
}