//! # Overview of main functions
//!
//! This is the list of all main functions. 
//!
//! ## Instantiation
//!
//! [instantiate()][fcgsales::Fcgsales::instantiate]
//! Creates a new Fcgsales instance.
//!
//! ## exchange
//!
//! [register()][fcgsales::Fcgsales::exchange]
//! Exchange 
//! 
//! ## Cancel
//!
//! [register()][fcgsales::Fcgsales::cancel_escrow]
//! Cancel an Escrow
//! 
//! ## Withdraw
//!
//! [register()][fcgsales::Fcgsales::Withdraw]
//! Withdraw a completed sale
//! 

use scrypto::prelude::*;

/// this is to contain data about an offer
#[derive(ScryptoSbor, NonFungibleData)]
pub struct EscrowData {
    pub requested_resource: ResourceAddress,
    pub requested_amount: Decimal,
    pub offered_resource: ResourceAddress              
}

#[blueprint]
mod fcgsales {

    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
            manager => updatable_by: [admin, OWNER];
            customer => updatable_by: [manager, admin, OWNER];
        },
        methods {
            exchange => PUBLIC;
            withdraw_resource => PUBLIC;
            cancel_escrow => PUBLIC;
        }
    }

    /// Data managed by the blueprint
    struct Fcgsales<> {
        escrow_vault: NonFungibleVault,
        completed_sale: Vault,
        requested_resource: ResourceAddress,
        requested_amount: Decimal,
        nft_manager: ResourceManager,
    }

    impl Fcgsales {

        /// Creates a new ready-to-use Fcgsales for an Escrow
        /// 
        /// This create also:
        ///   - a NFT manager to exchange or cancel the NFT representing the Escrow 
        /// 
        /// Returns a tuple containing:
        /// - The component address of the instantiated and globalized Fcgsales
        /// - The NonFungibleBucket containing an NFT representing the Escrow
        /// 
        pub fn instantiate(
                    requested_resource: ResourceAddress,
                    requested_amount: Decimal,
                    offered_resource: NonFungibleBucket
                ) 
            -> (Global<Fcgsales>, NonFungibleBucket) {


            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Fcgsales::blueprint_id());  
                
            // Create a resourceManager to manage EscrowData NFT
            let nft_manager =
                ResourceBuilder::new_ruid_non_fungible::<EscrowData>(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "FCG Sales EscrowData NFT", locked;
                        "symbol" => "FCG Sales EscrowData", locked;
                        "description" => "An NFT containing information about an Offer", locked;
                    }
                ))
                .mint_roles(mint_roles!(
                    minter => rule!(allow_all);
                    minter_updater => rule!(require(global_caller(component_address)));
                ))              
                .burn_roles(burn_roles!(
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => OWNER;
                ))    
                .create_with_no_initial_supply();

            info!("Minting an Escrow NFT");    
            let nft = 
                nft_manager
                .mint_ruid_non_fungible(
                    EscrowData {
                        requested_resource: requested_resource,
                        requested_amount: requested_amount,
                        offered_resource: offered_resource.resource_address()
                    }
                ).as_non_fungible();       

      
            // Instantiate a new component by storing the offered resource in a Vault
            let component = 
                Self {
                    escrow_vault: NonFungibleVault::with_bucket(offered_resource),
                    completed_sale: Vault::new(requested_resource),
                    requested_resource: requested_resource,
                    requested_amount: requested_amount,
                    nft_manager: nft_manager
                }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Updatable(rule!(require(
                    global_caller(component_address)
                ))))              
                .with_address(address_reservation)
                .globalize();
 
            return (component, nft);
        }

        /// This is for exchanging the offered resource with a bucket of fungibles
        /// 
        /// resource address and amount of the offer needs to be checked
        /// 
        /// Arguments:
        /// - `bucket_of_resource`: This is the offered bucket
        ///
        /// Returns:
        /// - the offered resource
        /// - bucket in exceed
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `escrow/exchange.rtm`
        /// ```text
        #[doc = include_str!("../escrow/exchange.rtm")]
        /// ```      
        pub fn exchange(&mut self, mut bucket_of_resource: Bucket) -> (NonFungibleBucket, Bucket) {
            info!("Exchange running  "); 

            //resource address needs to be checked
            let requested_resource = bucket_of_resource.resource_address();
            info!("Resource received: {:?} ", requested_resource); 
            
            assert!(self.requested_resource == requested_resource,
                "Resource Requested is different from what you provide!"
            );

            let requested_amount = bucket_of_resource.amount();
            //resource amount need to be enough
            assert!(
                bucket_of_resource.amount() >= self.requested_amount,
                "Amount of resources provided is not enough for a successfull exchange!"
            );
            //take the requested amount
            self.completed_sale.put(bucket_of_resource.take(requested_amount));

            //return the offered resource
            return (self.escrow_vault.take_all(),bucket_of_resource);
        }

        /// This is to withdraw the collected resource in exchange for the offered resource
        /// 
        /// 
        /// Arguments:
        /// - `escrow_nft`: the NFT representing the offer deposited in the component
        ///
        /// Returns, when escrow is completed:
        /// - the requested resource and amount
        /// 
        /// Returns, when escrow is not completed:
        /// - the same NFT that has been sent in
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `escrow/exchange.rtm`
        /// ```text
        #[doc = include_str!("../escrow/exchange.rtm")]
        /// ```      
        pub fn withdraw_resource(&mut self, escrow_nft: NonFungibleBucket) -> (Option<Bucket>,Option<NonFungibleBucket>) {

            let _escrow_data: EscrowData = escrow_nft.non_fungible().data();
            info!("Withdraw Amount: {:?} of: {:?} ", _escrow_data.requested_amount, _escrow_data.requested_resource);   

            match self.completed_sale.is_empty() {
                false => {
                    //burn the nft
                    escrow_nft.burn();
                    //return the collected tokens
                    info!("Amount returned: {:?} ", self.completed_sale.amount());   
                    return (Some(self.completed_sale.take_all()), None);
                }
                true => {
                    info!("Escrow has not been completed ! ");
                    return (None, Some(escrow_nft));
                }
            }
        }

        /// This is for canceling an escrow
        /// 
        /// Arguments:
        /// - `escrow_nft`: the NFT representing the offer deposited in the component
        ///
        /// Returns, when escrow is completed:
        /// - the same NFT that has been sent in
        /// 
        /// Returns, when escrow is not completed:
        /// - the offered resource and amount at the time of creating the escrow
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone
        ///
        /// **Transaction manifest:**
        /// `escrow/cancel_escrow.rtm`
        /// ```text
        #[doc = include_str!("../escrow/cancel_escrow.rtm")]
        /// ``` 
        pub fn cancel_escrow(&mut self, escrow_nft: NonFungibleBucket) -> Option<NonFungibleBucket> {

            let _escrow_data: EscrowData = escrow_nft.non_fungible().data();

            info!("Cancel Escrow: {:?} of: {:?} ", _escrow_data.requested_amount, _escrow_data.requested_resource);   

            match self.completed_sale.is_empty() {
                false => {
                    assert!(true == false, "Escrow is completed and it is not cancelable anymore!");
                    Some(escrow_nft)
                }
                true => {
                    info!("Escrow has not been completed and you cancel ! ");
                    escrow_nft.burn();
                    Some(self.escrow_vault.take_all())
                }
            }

        }

        pub fn assert_resource(res_addr: ResourceAddress, expect_res_addr: ResourceAddress){
            assert!(res_addr == expect_res_addr, "Incorrect resource passed in for interacting with the component!");
        }

    }
}