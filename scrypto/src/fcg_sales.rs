//! # Overview of main functions
//!
//! This is the list of all main functions. 
//!
//! ## Instantiation
//!
//! [instantiate()][fcgsales::Fcgsales::instantiate]
//! Creates a new Fcgsales instance.
//!
//! ## Send Offer
//!
//! [register()][fcgsales::Fcgsales::send_offer]
//! Send an Offer to a Customer
//! 
//! ## Cancel Offer
//!
//! [register()][fcgsales::Fcgsales::cancel_offer]
//! Cancel an Offer to a Customer
//! 
//! ## Accept Offer
//!
//! [register()][fcgsales::Fcgsales::accept_offer]
//! Accept an Offer to a Customer
//! 
//! ## Refuse Offer
//!
//! [register()][fcgsales::Fcgsales::refuse_offer]
//! Refuse an Offer to a Customer
//! 
//! # Overview of secondary functions
//!
//! This is the list of all the functions needed to setup, configure and manage the dApp functionalities
//! 
//! 

use scrypto::prelude::*;

/// this is to contain data about an offer
#[derive(ScryptoSbor, NonFungibleData)]
pub struct EscrowData {
    pub requested_resource: ResourceAddress,
    pub requested_amount: Decimal,
    pub offered_resource: ResourceAddress              
}


#[derive(ScryptoSbor, NonFungibleData)]
pub struct EscrowBadge {
    offered_resource: ResourceAddress
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
    // nft_manager: ResourceManager,                            -> Resource Manager for minting/updating EscrowData NFT
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

            // let global_caller_badge_rule = rule!(require(global_caller(component_address))); 
                
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

            //mint an NFT
            // let escrow_nft = EscrowData {
            //     requested_resource: requested_resource,
            //     requested_amount: requested_amount,
            //     offered_resource: offered_resource.resource_address()
            // };

            info!("Minting an Escrow NFT");
            // let nft: NonFungibleBucket = scrypto::prelude::NonFungibleBucket(nft_manager
            //     .mint_ruid_non_fungible(escrow_nft));
            // let nft: Bucket = nft_manager
            //     .mint_ruid_non_fungible(escrow_nft);         

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

        /// This creates and send a new offer to a customer
        /// The offer is represented by an hash that is tied to a PDF document that has been sent to the customer separately
        /// 
        /// 
        /// Arguments:
        /// - `hash_pdf`: This is the hash of the PDF document that contains the commercial offer 
        /// - `expiry_date`: Expiry date of the offer
        /// - `customer_account`: Account where this NFT will be sent (not needed)
        ///
        /// Returns 'Bucket':
        /// - the EscrowData NFT
        ///
        /// ---
        ///
        /// **Access control:** Can be called by an Admin or by a Manager.
        ///
        /// **Transaction manifest:**
        /// `fcgsales/send_offer_as_manager.rtm`
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
            return (self.escrow_vault.take(1),bucket_of_resource);
        }

        pub fn withdraw_resource(&mut self, escrow_nft: NonFungibleBucket) -> (Bucket,Option<NonFungibleBucket>) {

            let escrow_data: EscrowData = escrow_nft.non_fungible().data();

            info!("Withdraw Amount: {:?} of: {:?} ", escrow_data.requested_amount, escrow_data.requested_resource);   

            match self.completed_sale.is_empty() {
                false => {
                    //burn the nft
                    escrow_nft.burn();
                    //return the collected tokens
                    return (self.completed_sale.take_all(), None);
                }
                true => {
                    info!("Escrow has not been completed ! ");
                    return (Bucket::new(escrow_data.requested_resource), Some(escrow_nft));
                }
            }
        }

        /// This is for accepting an offer
        /// 
        /// Arguments:
        /// - `offer_data_proof`: the EscrowData NFT Proof 
        ///
        /// Returns 'None':
        ///
        /// ---
        ///
        /// **Access control:** Can be called by a Customer Only.
        ///
        /// **Transaction manifest:**
        /// `fcgsales/accept_offer.rtm`
        /// ```text
        #[doc = include_str!("../escrow/cancel_escrow.rtm")]
        /// ``` 
        pub fn cancel_escrow(&mut self, escrow_nft: NonFungibleBucket) -> Option<NonFungibleBucket> {

            let _escrow_data: EscrowData = escrow_nft.non_fungible().data();

            info!("Cancel Escrow: {:?} of: {:?} ", _escrow_data.requested_amount, _escrow_data.requested_resource);   

            match self.completed_sale.is_empty() {
                false => {
                    assert!(true == false, "Escrow is completed and it is not cancelable anymore!");
                    None
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


        // pub fn accept_offer(&mut self, offer_data_proof: NonFungibleProof)  {
        //     let current_epoch = Decimal::from(Runtime::current_epoch().number());
            
        //     // Update the state of the Offer
        //     let offer_data_proof = offer_data_proof.skip_checking();
        //     let nft_local_id: NonFungibleLocalId = offer_data_proof.as_non_fungible().non_fungible_local_id();
        //     let mut nfdata: EscrowData = ResourceManager::from(offer_data_proof.resource_address()).get_non_fungible_data(&nft_local_id);

        //     info!("Accepting an offer for this pdf {:?} with this expiry date {:?} at epoch  {:?} ",nfdata.hash_pdf, nfdata.expiry_date, current_epoch);

        //     assert!(nfdata.state == "NEW", "Offer is not acceptable anymore!");
        //     assert!(nfdata.expiry_date >= current_epoch, "Offer is expired!");
        //     self.nft_manager.update_non_fungible_data(&nft_local_id, "state", "ACCEPTED");   
        //     self.nft_manager.update_non_fungible_data(&nft_local_id, "accepted_timestamp", current_epoch);   

        //     //emit the event
        //     nfdata.state = "ACCEPTED".to_string();
        // }

    }
}