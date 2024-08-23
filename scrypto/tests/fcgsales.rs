
use scrypto_test::prelude::*;



#[test]
fn test_fcg_sales() {
    // Setup the environment
    let mut ledger = LedgerSimulatorBuilder::new().without_kernel_trace().without_receipt_substate_check().build();

    // Create an account
    let (public_key, _private_key, account) = ledger.new_allocated_account();
    // Create another account for a manager
    let (_manager_public_key1, _manager_private_key1, _manager_account1) = ledger.new_allocated_account();
    // Create another account for a customer
    let (_customer_public_key1, _customer_private_key1, customer_account1) = ledger.new_allocated_account();    
    let (_customer_public_key2, _customer_private_key2, _customer_account2) = ledger.new_allocated_account();
    
    // Create an owner and an admin badge
    // let owner_badge = ledger.create_fungible_resource(dec!(1), 1, account);
    // let admin_badge = ledger.create_fungible_resource(dec!(1), 1, account);
    // println!("owner = {:?}\n", owner_badge);
    // println!("admin_badge = {:?}\n", admin_badge);

    // Publish package
    let package_address = ledger.compile_and_publish(this_package!());

    // Test the `instantiate` function.
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "Fcgsales",
            "instantiate",
            ()
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    let receipt: TransactionReceiptV1 = ledger.execute_manifest(
        manifest,
        vec![],
    );
    println!(" Receipt of the instantiate = {:?}\n", receipt);
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    let _owner_badge = receipt.expect_commit(true).new_resource_addresses()[0];
    let admin_badge = receipt.expect_commit(true).new_resource_addresses()[1];
    let nft_manager = receipt.expect_commit(true).new_resource_addresses()[4];
    let manager_badge_resource_manager = receipt.expect_commit(true).new_resource_addresses()[3];
    let customer_badge_resource_manager = receipt.expect_commit(true).new_resource_addresses()[4];    


    println!(" NFT Manager = {:?}\n", nft_manager);
    println!(" Manager = {:?}\n", manager_badge_resource_manager);
    println!(" Customer = {:?}\n", customer_badge_resource_manager);

    //Send an offer
    let offer_amount = dec!(400);
    let expiry_date = dec!(3000);
    let receipt = ledger.execute_manifest(
        send_offer_as_admin(component, account, admin_badge, customer_account1, "hash_pdf".to_string(), expiry_date, offer_amount),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("Receipt from send_offer {:?}\n", receipt);
    receipt.expect_commit_success();

    //Send a badge to a Customer Account to let it accept an offer
    let receipt = ledger.execute_manifest(
        mint_customer_badge(component, account, admin_badge, customer_account1, "azienda1".to_string()),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("Receipt from mint_customer_badge {:?}\n", receipt);
    receipt.expect_commit_success();

    //TODO 
    // fetch the GlobalId of the Manager's Badge to execute 'send_offer_as_manager' function

    //TODO 
    // same for the customer, fetch the GlobalId of the Customer's Badge to execute 'accept_offer' function

    //Accept an offer
    let offerdata_nft_global_id = NonFungibleGlobalId::new(nft_manager, NonFungibleLocalId::integer(1));
    let customer_badge_global_id = NonFungibleGlobalId::new(customer_badge_resource_manager, NonFungibleLocalId::integer(4));
    let receipt = ledger.execute_manifest(
        accept_offer(component, customer_account1, customer_badge_global_id, offerdata_nft_global_id),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("Receipt from accept_offer {:?}\n", receipt);
    receipt.expect_commit_success();


}

/// Send an offer by using an Admin badge
/// 
/// 
fn send_offer_as_admin(component: ComponentAddress, admin_account: ComponentAddress, admin_badge: ResourceAddress, customer_account: ComponentAddress, hash_pdf: String, expiry_date: Decimal, offer_amount: Decimal) -> TransactionManifestV1 {
    ManifestBuilder::new()
    .lock_fee_from_faucet() 
    .create_proof_from_account_of_amount(admin_account, admin_badge, dec!(1))
    .call_method_with_name_lookup(  // #1
        component,
        "send_offer",
        |_lookup| (  
            hash_pdf, // #1
            expiry_date, // #2
            offer_amount,
            customer_account
        )
    )
    .try_deposit_entire_worktop_or_abort(customer_account,  None)    
    .build()
}    

/// Send an offer by using a Manager NonFungible badge
/// 
/// 
fn _send_offer_as_manager(component: ComponentAddress, manager_account: ComponentAddress, global_id: NonFungibleGlobalId, customer_account: ComponentAddress, hash_pdf: String, expiry_date: Decimal, offer_amount: Decimal) -> TransactionManifestV1 {
    ManifestBuilder::new()
    .lock_fee_from_faucet() 
    .create_proof_from_account_of_non_fungible(manager_account, global_id)
    .call_method_with_name_lookup(  // #1
        component,
        "send_offer",
        |_lookup| (  
            hash_pdf, // #1
            expiry_date, // #2
            offer_amount,
            customer_account // #4
        )
    )
    .try_deposit_entire_worktop_or_abort(customer_account, None)    
    .build()
}    

/// Accept an offer by using a Customer NonFungible badge
/// 
/// 
fn accept_offer(component: ComponentAddress, customer_account: ComponentAddress, customer_badge_global_id: NonFungibleGlobalId, offerdata_nft_global_id: NonFungibleGlobalId) -> TransactionManifestV1 {
    ManifestBuilder::new()
    .lock_fee_from_faucet() 
    .create_proof_from_account_of_non_fungible(customer_account, customer_badge_global_id)
    .create_proof_from_account_of_non_fungible(customer_account, offerdata_nft_global_id)
    .pop_from_auth_zone("offer_data")
    .call_method_with_name_lookup(  // #1
        component,
        "accept_offer",
        |lookup| (  
            lookup.proof("offer_data"),
        )
    )
    .build()
}    

/// Ming a Customer badge to let it accept an offer
/// 
/// 
fn mint_customer_badge(component: ComponentAddress, admin_account: ComponentAddress, admin_badge: ResourceAddress, customer_account: ComponentAddress, username: String) -> TransactionManifestV1 {
    ManifestBuilder::new()
    .lock_fee_from_faucet() 
    .create_proof_from_account_of_amount(admin_account, admin_badge, dec!(1))
    .call_method_with_name_lookup(  // #1
        component,
        "mint_customer_badge",
        |_lookup| (  
            username,
        )
    )
    .try_deposit_entire_worktop_or_abort(customer_account,  None)    
    .build()
}    



