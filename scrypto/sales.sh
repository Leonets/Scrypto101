clear 
set -e

export xrd=resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3

echo "Resetting environment"
resim reset
export owner_account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo "Owner Account = " $owner_account

echo "XRD = " $xrd

echo "Publishing dapp"
export dapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $dapp_package

echo "Instantiate dapp"
output=`resim call-function $dapp_package Fcgsales instantiate  | awk '/Component: |Resource: / {print $NF}'`
export component=`echo $output | cut -d " " -f1`
export owner_badge=`echo $output | cut -d " " -f2`
export admin_badge=`echo $output | cut -d " " -f3`
export manager=`echo $output | cut -d " " -f4`
export customer=`echo $output | cut -d " " -f5`
export nft_manager=`echo $output | cut -d " " -f6`

echo "Export component test"
export component_test=component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh

echo "Show instantiate output"
echo 'output = '$output

echo 'component = '$component
echo 'owner_badge = '$owner_badge
echo 'admin_badge = '$admin_badge
echo 'manager = ' $manager
echo 'customer = ' $customer
echo 'nft_manager = ' $nft_manager

echo ' '
echo 'account = ' $owner_account
echo 'xrd = ' $xrd
echo 'test faucet for lock fee = ' $component_test
echo ' '


# Create a Customer Account 

# Run the command and capture the output
output=$(resim new-account)
# Extract the Customer Account component address
export customer_account=$(echo "$output" | awk -F': ' '/Account component address/ {print $2}')
# Extract the Customer Public key
export public_key=$(echo "$output" | awk -F': ' '/Public key/ {print $2}')
# Extract the Customer Private key
export private_key=$(echo "$output" | awk -F': ' '/Private key/ {print $2}')
# Extract the Customer Owner Badge
export customer_owner_badge=$(echo "$output" | awk -F': ' '/Owner badge/ {print $2}')
# Display the values (optional)
echo 'Customer Account = ' $customer_account
echo 'Public Key = ' $public_key
echo 'Private Key = ' $private_key
echo 'Customer Owner Badge = ' $customer_owner_badge

# Create a manager Account 

# export manager_account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
# echo 'Manager Account = ' $manager_account

# Run the command and capture the output
output=$(resim new-account)
# Extract the Manager Account component address
export manager_account=$(echo "$output" | awk -F': ' '/Account component address/ {print $2}')
# Extract the Manager Public key
export manager_public_key=$(echo "$output" | awk -F': ' '/Public key/ {print $2}')
# Extract the Manager Private key
export manager_private_key=$(echo "$output" | awk -F': ' '/Private key/ {print $2}')
# Extract the Manager Owner Badge
export manager_owner_badge=$(echo "$output" | awk -F': ' '/Owner badge/ {print $2}')
# Display the values (optional)
echo 'Manager Account = ' $manager_account
echo 'Public Key = ' $manager_public_key
echo 'Private Key = ' $manager_private_key
echo 'Manager Owner Badge = ' $manager_owner_badge


# Show Accounts 
echo ' > owner account'
resim show $owner_account
echo ' > customer account'
resim show $customer_account
echo ' > manager account'
resim show $manager_account

# Show Badges
echo ' > owner'
resim show $owner_badge
echo ' > admin'
resim show $admin_badge
echo ' > manager'
resim show $manager
echo ' > customer'
resim show $customer
echo ' > nft_manager'
resim show $nft_manager

# Mint and send a badge to a Customer (to enable it to accept/refuse offers) 
export customer_name=Azienda1
echo '>>> Mint a Customer Badge'
resim run fcgsales/mint_customer_badge.rtm

# Mint and send a badge to a Manager (to enable it to send/cancel offers) 
export manager_name=Leonardo
echo '>>> Mint a Manager Badge'
resim run fcgsales/mint_manager_badge.rtm

# Show Manager and Customer account with Badges
echo ' > manager'
resim show $manager_account
echo ' > customer'
resim show $customer_account


# Run the command and capture the output and get NFT LocalId to be used for sending an offer
customer_fungible=$(resim show $manager_account)
# Extract the value inside the braces after "Fcgsales Manager Badge (Fcgsales Manager)"
export manager_badge_id=$(echo "$customer_fungible" | awk '/Fcgsales Manager Badge/ {getline; print $2}' | tr -d '{}')
# Display the extracted value
echo 'Manager Badge = ' $manager_badge_id

# Send an Offer to a Customer
# TODO - The offer is being sent as an ADMIN
# TODO - Here it is needed to change Default account to a Manager
# TODO - Here it is needed to fetch NFT Local ids
export hash_pdf=hash_pdf
export expiry_date=100
export offer_amount=400
export account=$owner_account
export amount=1
# echo '>>> Send Offer'
# resim run fcgsales/send_offer.rtm

# TODO - This is an offer sent as a Manager
# Change Default account to a Manager
echo ' > I need to change and set the default account as the manager'
resim set-default-account $manager_account $manager_private_key $manager_owner_badge
echo '>>> Send Offer as a Manager'
resim run fcgsales/send_offer_as_manager.rtm




# Show Accounts 
echo ' > owner account'
resim show $owner_account
echo ' > customer account'
resim show $customer_account
echo ' > manager account'
resim show $manager_account

# Run the command and capture the output and get NFT LocalId to be used for accepting an offer
fungible=$(resim show $customer_account)
# Extract the value inside the braces after "FCG Sales OfferData NFT (FCG Sales OfferData)"
nft_data=$(echo "$fungible" | awk '/Sales OfferData NFT/ {getline; print $2}' | tr -d '{}')
# Display the extracted value
echo 'Sales OfferData NFT Data = ' $nft_data

# Extract the value inside the braces after "Fcgsales Customer Badge (Fcgsales Customer)"
export customer_badge_id=$(echo "$fungible" | awk '/Fcgsales Customer Badge/ {getline; print $2}' | tr -d '{}')
# Display the extracted value
echo 'Customer Badge = ' $customer_badge_id





# # TODO - Cancel the offer
# export vaultAddress=abc
# export fungibleId={$nft_data}
# echo '>>> Accept Offer ' $fungibleId
# resim run fcgsales/recall_offer.rtm

# # TODO - Check if the NFT has been recalled
# echo ' > customer account'
# resim show $customer_account





# Change Default account to a Customer
echo ' > I need to change and set the default account as the customer'
resim set-default-account $customer_account $private_key $customer_owner_badge

# Accept the first offer
export fungibleId={$nft_data}
echo '>>> Accept Offer ' $fungibleId
resim run fcgsales/accept_offer.rtm
