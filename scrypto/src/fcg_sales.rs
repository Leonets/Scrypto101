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
//! ## Mint Offer Manager
//!
//! [mint_manager_badge()][fcgsales::Fcgsales::mint_manager_badge]
//! Function for minting a new badge for then allowing a Manager to create offer for customer
//! 
//! ## Mint Customer Badge
//!
//! [mint_customer_badge()][fcgsales::Fcgsales::mint_customer_badge]
//! Function for minting a new badge for then allowing a Customer to accept/refuse offer
//! 

use scrypto::prelude::*;
use scrypto_avltree::AvlTree;

/// this is to contain data about an offer
#[derive(ScryptoSbor, NonFungibleData)]
pub struct OfferData {
    pub hash_pdf: String,
    #[mutable]
    pub expiry_date: Decimal,
    #[mutable]
    pub state: String,    
    pub create_timestamp: Decimal,
    #[mutable]
    pub accepted_timestamp: Decimal,
    #[mutable]
    pub refused_timestamp: Decimal,
    pub offer_amount: Decimal                
}

/// this is to contain the username of a Manager Member
#[derive(NonFungibleData, ScryptoSbor)]
struct ManagerBadge {
    username: String
}

/// this is to contain the username of a Customer Member
#[derive(NonFungibleData, ScryptoSbor)]
struct CustomerBadge {
    username: String
}


#[derive(ScryptoSbor, ScryptoEvent)]
struct AcceptedOfferEvent {
    offer: OfferData,
    epoch: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct RefusedOfferEvent {
    offer: OfferData,
    epoch: Decimal,
}

#[blueprint]
#[events(AcceptedOfferEvent, RefusedOfferEvent)]
mod fcgsales {
    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
            manager => updatable_by: [admin, OWNER];
            customer => updatable_by: [manager, admin, OWNER];
        },
        methods {
            send_offer => restrict_to: [manager, admin, OWNER];
            cancel_offer => restrict_to: [manager, admin, OWNER];
            accept_offer => restrict_to: [customer];
            refuse_offer => restrict_to: [customer];
            mint_manager_badge => restrict_to: [admin, OWNER];
            mint_customer_badge => restrict_to: [manager, admin, OWNER];
        }
    }

    /// Data managed by the blueprint
    // nft_manager: ResourceManager,                            -> Resource Manager for minting/updating OfferData NFT
    // manager: AvlTree<u16, NonFungibleLocalId>,               -> List of manager members NonFungibleLocalId
    // customer: AvlTree<u16, NonFungibleLocalId>,              -> List of customer members NonFungibleLocalId
    // manager_badge_resource_manager: ResourceManager,         -> Resource manager for minting/burning/recalling manager badges
    // customer_badge_resource_manager: ResourceManager,        -> Resource manager for minting/burning/recalling a customer badges
    struct Fcgsales<> {
        nft_manager: ResourceManager,
        manager: AvlTree<u16, NonFungibleLocalId>,
        customer: AvlTree<u16, NonFungibleLocalId>,
        manager_badge_resource_manager: ResourceManager,
        customer_badge_resource_manager: ResourceManager,
    }

    impl Fcgsales {

        /// Creates a new ready-to-use Fcgsales, returning also an owner and an admin badge
        /// 
        /// This create also:
        ///   - resource managers to manage customer and manager badges
        ///   - a NFT manager to mint/recall OfferData 
        ///   - two separate containers to store customer and manager list of NonFungibleLocalId (not needed)
        /// 
        /// Returns a tuple containing:
        /// - The component address of the instantiated and globalized Fcgsales
        /// - An Owner badge 
        /// - An Admin badge 
        /// 
        pub fn instantiate() -> (Global<Fcgsales>, FungibleBucket, FungibleBucket) {

            //container
            let manager: AvlTree<u16, NonFungibleLocalId> = AvlTree::new();
            let customer: AvlTree<u16, NonFungibleLocalId> = AvlTree::new();

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Fcgsales::blueprint_id());

            //owner badge
            let owner_badge = 
                ResourceBuilder::new_fungible(OwnerRole::None)
                    .metadata(metadata!(init{
                        "name"=>"FCG Sales Owner badge", locked;
                        "symbol" => "FCG Sales Owner", locked;
                        "description" => "A badge to be used for some extra-special administrative function", locked;
                    }))
                    .divisibility(DIVISIBILITY_NONE)
                    .mint_initial_supply(1);

            //admin badge
            let admin_badge = 
                ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(require(
                    owner_badge.resource_address()
                ))))
                    .metadata(metadata!(init{
                        "name"=>"FCG Sales Admin badge", locked;
                        "symbol" => "FCG Sales Admin", locked;
                        "description" => "A badge to be used for some special administrative function", locked;
                    }))
                    .divisibility(DIVISIBILITY_NONE)
                    .mint_initial_supply(1);


            // Create a resourceManager to manage Manager Badges
            // A manager badge can be created by the component, by the component owner badge or by an admin 
            let manager_resource_manager: ResourceManager = 
                ResourceBuilder::new_integer_non_fungible::<ManagerBadge>(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                )))
                .metadata(metadata!(init{
                    "name" => "Fcgsales Manager Badge", locked;
                    "symbol" => "Fcgsales Manager", locked;
                    "description" => "A badge to be used for some manager function", locked;
                }))
                .mint_roles(mint_roles! (
                        minter => rule!(
                                require(global_caller(component_address))
                                || require(owner_badge.resource_address())
                                || require(admin_badge.resource_address())
                        );
                        minter_updater => OWNER;
                ))
                .burn_roles(burn_roles! (
                    burner => rule!(
                            require(global_caller(component_address))
                            || require(owner_badge.resource_address())
                            || require(admin_badge.resource_address())
                    );
                    burner_updater => OWNER;
                ))
                .recall_roles(recall_roles! {
                    recaller => rule!(
                            require(global_caller(component_address))
                            || require(owner_badge.resource_address())
                            || require(admin_badge.resource_address())
                    );
                    recaller_updater => OWNER;
                })
            .create_with_no_initial_supply();           


            // Create a resourceManager to manage Customer Badges
            // A Customer badge can be created by the component, by an admin or by a manager 
            let customer_resource_manager: ResourceManager = 
                ResourceBuilder::new_integer_non_fungible::<CustomerBadge>(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                        || require(manager_resource_manager.address())
                )))
                .metadata(metadata!(init{
                    "name" => "Fcgsales Customer Badge", locked;
                    "symbol" => "Fcgsales Customer", locked;
                    "description" => "A badge to be used for some customer function", locked;
                }))
                .mint_roles(mint_roles! (
                    minter => rule!(
                                require(global_caller(component_address))
                                || require(admin_badge.resource_address())
                                || require(manager_resource_manager.address()));
                    minter_updater => OWNER;
                ))
                .burn_roles(burn_roles! (
                    burner => rule!(
                        require(global_caller(component_address))
                        || require(admin_badge.resource_address())
                        || require(manager_resource_manager.address()));
                    burner_updater => OWNER;
                ))
                .recall_roles(recall_roles! {
                    recaller => rule!(
                        require(global_caller(component_address))
                        || require(admin_badge.resource_address())
                        || require(manager_resource_manager.address()));
                    recaller_updater => OWNER;
                })
            .create_with_no_initial_supply();             
                

            // Create a resourceManager to manage OfferData NFT
            // This NFT is also burnable in the scope of this specific blueprint customized for this challenge
            // Mint is available only from the component
            let nft_manager =
                ResourceBuilder::new_ruid_non_fungible::<OfferData>(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                )))
                .metadata(metadata!(
                    init {
                        "name" => "FCG Sales OfferData NFT", locked;
                        "symbol" => "FCG Sales OfferData", locked;
                        "description" => "An NFT containing information about an Offer", locked;
                    }
                ))
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(require(global_caller(component_address)));
                ))
                .recall_roles(recall_roles!(
                    recaller => rule!(
                        require(global_caller(component_address))
                        || require(admin_badge.resource_address())
                        || require(manager_resource_manager.address()));
                    recaller_updater => OWNER;
                ))                
                .burn_roles(burn_roles!(
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => OWNER;
                ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                    non_fungible_data_updater => rule!(require(global_caller(component_address)));
                    non_fungible_data_updater_updater => OWNER;
                ))           
                .create_with_no_initial_supply();
      

            // Populate a Fcgsales struct and instantiate a new component
            // 
            let component = 
                Self {
                    nft_manager: nft_manager,
                    manager: manager,
                    customer: customer,
                    manager_badge_resource_manager: manager_resource_manager,
                    customer_badge_resource_manager: customer_resource_manager,
                }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Updatable(rule!(require(
                    owner_badge.resource_address()
                ))))
                .enable_component_royalties(component_royalties! {
                    // The roles section is optional, if missing, all roles default to OWNER
                    roles {
                        royalty_setter => rule!(allow_all);
                        royalty_setter_updater => OWNER;
                        royalty_locker => OWNER;
                        royalty_locker_updater => rule!(deny_all);
                        royalty_claimer => OWNER;
                        royalty_claimer_updater => rule!(deny_all);
                    },
                    // Herein we are specifyng which functions generate a commission for the accounts
                    init {
                        send_offer => Free, locked;
                        cancel_offer => Free, locked;

                        accept_offer => Free, locked;
                        refuse_offer => Free, locked;

                        mint_manager_badge => Free, locked;
                        mint_customer_badge => Free, locked;
                    }
                })                
                .metadata(metadata!(
                    init {
                        "name" => "Fcgsales", locked;
                        "icon_url" => Url::of("https://fcgsales.eu/images/logo.jpg"), locked;
                        "description" => "FCG Sales SmartContract for digitalizing an offer service", locked;
                        "claimed_websites" =>  ["https://fcgsales.eu"], locked;
                    }
                ))
                //Herein we are specifying what does a role need to present a proof of itself
                .roles(roles!(
                    admin => rule!(require(admin_badge.resource_address()));
                    manager => rule!(require(manager_resource_manager.address()));
                    customer => rule!(require(customer_resource_manager.address()));
                ))
                .with_address(address_reservation)
                .globalize();
 
            return (component, admin_badge, owner_badge);
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
        /// - the OfferData NFT
        ///
        /// ---
        ///
        /// **Access control:** Can be called by an Admin or by a Manager.
        ///
        /// **Transaction manifest:**
        /// `fcgsales/send_offer_as_manager.rtm`
        /// ```text
        #[doc = include_str!("../fcgsales/send_offer_as_manager.rtm")]
        /// ```        
        pub fn send_offer(&mut self, hash_pdf: String, expiry_date: Decimal, offer_amount: Decimal, _customer_account: Global<Account>) -> Bucket {
            info!("Ready for minting an offer ");
            //mint an NFT
            let epoch = Decimal::from(Runtime::current_epoch().number());
            let offer = OfferData {
                hash_pdf: hash_pdf.clone(),
                expiry_date: expiry_date,
                state: "NEW".to_string(),    
                create_timestamp: epoch,
                accepted_timestamp: dec!(0),
                refused_timestamp: dec!(0),
                offer_amount: offer_amount   
            };

            info!("Minting an offer ");
            
            let nft = self
            .nft_manager
                .mint_ruid_non_fungible(offer);

            info!("Sending an offer for this pdf {:?} with this expiry date {:?} to this account {:?}  ",hash_pdf, expiry_date, _customer_account);

            nft
        }

        /// This removes the offer NFT from the customer account
        /// 
        /// 
        /// Arguments:
        /// - `offer_id`: NonFungibleLocalId of the NFT to be recalled
        ///
        /// Returns 'None':
        ///
        /// ---
        ///
        /// **Access control:** Can be called by an Admin or by a Manager.
        ///
        /// **Transaction manifest:**
        /// `fcgsales/recall_offer.rtm`
        /// ```text
        #[doc = include_str!("../fcgsales/recall_offer.rtm")]
        /// ```    
        pub fn cancel_offer(&mut self, _offer_id: NonFungibleLocalId)  {

            todo!();
        }

        /// This is for accepting an offer
        /// 
        /// Arguments:
        /// - `offer_data_proof`: the OfferData NFT Proof 
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
        #[doc = include_str!("../fcgsales/accept_offer.rtm")]
        /// ```        
        pub fn accept_offer(&mut self, offer_data_proof: NonFungibleProof)  {
            let current_epoch = Decimal::from(Runtime::current_epoch().number());
            
            // Update the state of the Offer
            let offer_data_proof = offer_data_proof.skip_checking();
            let nft_local_id: NonFungibleLocalId = offer_data_proof.as_non_fungible().non_fungible_local_id();
            let mut nfdata: OfferData = ResourceManager::from(offer_data_proof.resource_address()).get_non_fungible_data(&nft_local_id);

            info!("Accepting an offer for this pdf {:?} with this expiry date {:?} at epoch  {:?} ",nfdata.hash_pdf, nfdata.expiry_date, current_epoch);

            assert!(nfdata.state == "NEW", "Offer is not acceptable anymore!");
            assert!(nfdata.expiry_date >= current_epoch, "Offer is expired!");
            self.nft_manager.update_non_fungible_data(&nft_local_id, "state", "ACCEPTED");   
            self.nft_manager.update_non_fungible_data(&nft_local_id, "accepted_timestamp", current_epoch);   

            //emit the event
            nfdata.state = "ACCEPTED".to_string();
            Runtime::emit_event(AcceptedOfferEvent { offer: nfdata, epoch: current_epoch});
        }

        /// This is for refusing an offer
        /// 
        /// Arguments:
        /// - `offer_data_proof`: the OfferData NFT Proof 
        ///
        /// Returns 'None':
        ///
        /// ---
        ///
        /// **Access control:** Can be called by a Customer Only.
        ///
        /// **Transaction manifest:**
        /// `fcgsales/refuse_offer.rtm`
        /// ```text
        #[doc = include_str!("../fcgsales/refuse_offer.rtm")]
        /// ```      
        pub fn refuse_offer(&mut self, offer_data_proof: NonFungibleProof) {
            let current_epoch = Decimal::from(Runtime::current_epoch().number());

            // Update the state of the Offer
            let offer_data_proof = offer_data_proof.skip_checking();
            let nft_local_id: NonFungibleLocalId = offer_data_proof.as_non_fungible().non_fungible_local_id();
            let mut nfdata: OfferData = ResourceManager::from(offer_data_proof.resource_address()).get_non_fungible_data(&nft_local_id);
            
            info!("Refuse an offer for this pdf {:?} with this expiry date {:?} at epoch  {:?} ",nfdata.hash_pdf, nfdata.expiry_date, current_epoch);

            assert!(nfdata.state == "NEW", "Offer is not refusable anymore!");
            assert!(nfdata.expiry_date >= current_epoch, "Offer is expired!");
            self.nft_manager.update_non_fungible_data(&nft_local_id, "state", "REFUSED");   
            self.nft_manager.update_non_fungible_data(&nft_local_id, "refused_timestamp", current_epoch);   

            //emit the event
            nfdata.state = "REFUSED".to_string();
            Runtime::emit_event(RefusedOfferEvent { offer: nfdata, epoch: current_epoch});
        }

        /// Utility function: Mint a manager badge
        /// 
        /// Arguments:
        /// - `username`: Username that will be registered in the NFT
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin only.
        ///                    
        pub fn mint_manager_badge(&mut self, username: String) -> Bucket {
            let manager_badge_bucket: Bucket = self
                .manager_badge_resource_manager
                .mint_non_fungible(
                    &NonFungibleLocalId::integer((self.manager.get_length()+1).try_into().unwrap()),
                    ManagerBadge {
                        username: username.clone(),
                    });

            let id = manager_badge_bucket.as_non_fungible().non_fungible_local_id();
            let key = self.manager.get_length().to_u16().unwrap()+1; 
            info!("Saving staff badge with key : {:?} and id {:?} for the username: {:?}  ",key, id, username);
            self.manager.insert(key, id);

            manager_badge_bucket
        }

        /// Utility function: Mint a customer badge
        /// 
        /// Arguments:
        /// - `username`: Username that will be registered in the NFT
        /// ---
        ///
        /// **Access control:** Can be called by the Admin or the Customer only.
        ///                    
        pub fn mint_customer_badge(&mut self, username: String) -> Bucket {
            let customer_badge_bucket: Bucket = self
                .customer_badge_resource_manager
                .mint_non_fungible(
                    &NonFungibleLocalId::integer((self.customer.get_length()+1).try_into().unwrap()),
                    CustomerBadge {
                        username: username.clone(),
                    });

            let id = customer_badge_bucket.as_non_fungible().non_fungible_local_id();
            let key = self.customer.get_length().to_u16().unwrap()+1; 
            info!("Saving staff badge with key : {:?} and id {:?} for the username: {:?}  ",key, id, username);
            self.customer.insert(key, id);

            customer_badge_bucket
        }
    
    }
}