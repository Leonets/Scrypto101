# Scrypto 101 (Escrow)

here is the data of the deployed blueprint

Tx Manifest files are in stokenet/ directory

Transaction ID: txid_tdx_2_14p8q44ttwjm0gr8ld5esmt5lp64s8txvvzmc75f6dg9qkm279vns6y8x7j
Package address: package_tdx_2_1p4jsqfy3lzctyh7zl3qsttac3a9lsn9zqee8h4u04zen9a0x4fgl70

Non-Fungible resource created for the Escrow example
Transaction ID: txid_tdx_2_1dne8zrr7wxkp76w3lgu2ym8u8gftk993u9afjseuqkg2acv6pmeq36h2g5
Resource address: resource_tdx_2_1ntrl98xfxl0rz0ufhyhs03murl6aftrdlre78n5lk6zaje3cpelw6x

Component Instantiation
TX Id: txid_tdx_2_136xnnmzkq9nawt624rzz0sv5hshkuqnrsnv88rxnmaqr8tutp8jsm9jhhq

Created entities

    component_tdx_2_1crpnx76uprpm3qp8ymtfmhhp4sr8hqadatlff9amdqnldw3lgwe8wl
    resource_tdx_2_1nga8w8sz78vsxnghdwzmesxcdzawljladp5zp79j9twsfzhspramun

Cancel Escrow Tx:
    txid_tdx_2_1g4lqycelghyqxn98w2z7tdqpvdm6u8f9ckmezv8qvk9hah43z62sgqhk8y

New Escrow Component TxId:
    txid_tdx_2_1gj9s79ut7exmuqpydu03240lvmgttsezdeefxqun5wef4vl6zkdquaq5wm


Created entities
    component_tdx_2_1cqhmyfm65sk7pmg40jzt3m5awfg3ws39k56kzl8uq39thf3f73hw4z
    resource_tdx_2_1ngy3ryxcpvtf3ex0zn84eknlp26cppxk255q363x70rv69k5kfgnpc

Exchange Txid:
    txid_tdx_2_1tp63nf6yegr2yd4e9gwsnddy0mgluksu3cfeg6s5equclp225srschpu2v

Withdraw
    txid_tdx_2_1tswqf9cux7ragc43vepwjxz729ajrny3r4naaha5290t399p44wscwmr6t

Exchange TxId
    txid_tdx_2_1xy39s0u9aawwzcjdeg5wf6f8sq5nhg2tlzrchxtfl4n99pc3l70qrvh7ku

Withdraw
    txid_tdx_2_1qjkyjvf465c677ehk5qgaz0ex3vejafmh93h4tw3j5caq6jeqmvsske5pe

    

{1f0970228c6c33ca-e8b361a216e6dfff-3ff52a762360fddc-20aacd29cc612f69}    


Tx - Offer sent to the customer (as admin)
    txid_tdx_2_14kacet93j3g58kzl26hcwmqjcg75x4j0z2v5ru8ctc9srh3nf9sqen3j5h

Tx- Send a badge to a customer to let it accept the offer
    txid_tdx_2_1ll3hn3ylkz5nz9z7z8e3u9sdpv46k282gt7qhlmxv25hc8mzexyq55tv27

Tx - Offer accepted by the customer 
    txid_tdx_2_1p7xz9nhe0tdeqtcs7uv2y9pyldc6u70323s6sldjl6ufxs7qkwssx8keey


# Scrypto 101 (FCGSales)

here is the data of the deployed blueprint

Tx Manifest files are in stokenet/ directory

Transaction ID: txid_tdx_2_1fy2n40wcfz5nstukjvhdngrl54yuc02h7zggv3atvpzzslut0cys84y3c9
Package address: package_tdx_2_1ph2pl80stjfh6hlpfwukjkf9xzedh4ygprspkg2ps75dqxm3038efl

Created entities

    component_tdx_2_1cpkdxhvreprd6kwvztjmn2y0wj6qctnlpefeaz9vu9cp3ahr4l9xcs
    owner=resource_tdx_2_1th5mamrx8ck0nem8ktgpum9t6aa808nl8t9y5p0ak8z5n3q4jxefa9
    admin=resource_tdx_2_1t4nt68uuhm4xezgdjcp0sx283207d03mpml78sszsaxyjqnf3t2lep
    manager=resource_tdx_2_1nf7fqj8m7mzvp2uud8lujcjax64su0g3s9e64l9esnxdm2fd489p54
    customer=resource_tdx_2_1nfhascllr4sr33vdhwhlr4w5p6hst8rzdgnu94z3yqn59hrt9yhwjc
    nft=resource_tdx_2_1nfs8geg85lmhfp0hv76xmzls8grvkvhsxzj8g5vcgqtpslh73zuqfp

Tx - Offer sent to the customer (as admin)
    txid_tdx_2_14kacet93j3g58kzl26hcwmqjcg75x4j0z2v5ru8ctc9srh3nf9sqen3j5h

Tx- Send a badge to a customer to let it accept the offer
    txid_tdx_2_1ll3hn3ylkz5nz9z7z8e3u9sdpv46k282gt7qhlmxv25hc8mzexyq55tv27

Tx - Offer accepted by the customer 
    txid_tdx_2_1p7xz9nhe0tdeqtcs7uv2y9pyldc6u70323s6sldjl6ufxs7qkwssx8keey

Economic offer has been accepted 
https://stokenet-dashboard.radixdlt.com/nft/resource_tdx_2_1nfs8geg85lmhfp0hv76xmzls8grvkvhsxzj8g5vcgqtpslh73zuqfp%3A%7B615fd50395f8f74b-1b9134489b4b38a5-6eb470f3aeca1705-82e1afe1810ac7ab%7D

Off-chain work could be started


# FCG Sales

This dApp allows the component owner to create admins and then managers for tracking and register over a DLT the approvals of the economic offers that are represented by a PDF Document.

The use case is the following:
- a company want to send an economic offer to a customer
- the company manager prepare a PDF document with all the technical ad economic conditions
- the company manager create an hash of the document and send that to the customer by email
- the company manager send an OfferData NFT to the customer account
- the customer looks at the PDF Document and the approve/reject the offer

The OfferData NFT is stored in the DLT and its state represent the state of the offer.

Only a Manager can send an offer to a Customer, and only a Customer can approve/reject that.

The offer has an expiery date and it is recallable before its expiry date

# FCG Sales Implementation

The data structure containing the OfferData is defined as:

    pub hash_pdf: String,
    #[mutable]
    pub expiry_date: Decimal,
    #[mutable]
    pub state: String,    
    pub create_timestamp: Decimal,
    pub accepted_timestamp: Decimal,
    pub refused_timestamp: Decimal     

It worth have a look at this data structures:

OfferData

    hash_pdf:  -> hash of the PDF

    expiry_date:  -> expiry date

    state:  -> state of the offer


## Owner, Admin and Staff Badges

The component manages three types of profiles: Owner, Admin, Manager and Customer, each with different functionalities.

The Manager Badge is particularly noteworthy. An Admin can mint a Manager Badge and send it to a manager member using the Radix Wallet. 
Once received, the manager member can perform their allowed actions.

The same is for a Customer Badge

Customer and Manager member can also be removed because the badge is recallable!

# Documenting & Building & Testing

## Documentation 

You can run `cargo doc --no-deps --open --document-private-items` from the `scrypto` directory to create the documentation in the directory `scrypto\target\doc`, and `cargo doc --open` to have it opened it in your web browser

You can run `jsdoc js -d docs` from the `client` directory to create the documentation in the directory `client\docs` about the Javascript functions

## Package building

You can run `scrypto build` from the `scrypto` directory for building the packages for deploy

## Unit test

You can run `scrypto test` from the `scrypto` directory for testing the main functions


# Interacting with the FCGSales

You can have a look at the bash script to simulate some of the possible use-case for this blueprint:

 - scrypto/sales.sh : A simple test that executes the send and accept 


