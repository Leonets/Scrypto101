clear 
set -e

export xrd=resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3

echo "Resetting environment"
resim reset
export account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo "Owner Account = " $account

echo "XRD = " $xrd

echo "Publishing dapp"
export dapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $dapp_package

export non_fungible_global_id=$(resim new-simple-badge | sed -nr "s/NonFungibleGlobalId: ([[:alnum:]#:_]+)/\1/p")
export non_fungible_local_id=#1#
export escrow_non_fungible_local_id={0a6a31c373b8c4fb-eeb2b0562d4e3eb1-3a10ab15d5aac00d-bb062d005d1fafb0}

echo "Account before Instantiate dapp"
resim show $account

echo "Instantiate dapp"
output=`resim call-function $dapp_package Fcgsales instantiate $xrd 1000 $non_fungible_global_id | awk '/Component: |Resource: / {print $NF}'`
export component=`echo $output | cut -d " " -f1`
export escrow_nft=`echo $output | cut -d " " -f2`


echo "Export component test"
export component_test=component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh

echo "Show instantiate output"
echo 'output = '$output

echo 'component = '$component
echo 'escrow_nft = '$escrow_nft

resim show $escrow_nft

echo ' '
echo 'account = ' $account
echo 'xrd = ' $xrd
echo 'test faucet for lock fee = ' $component_test
echo ' '

echo 'Component Info'
resim show $component

echo 'Before Exchange'
resim show $account

# Accept the first offer
echo '>>> Exchange ' 
export amount=1000
resim run escrow/exchange.rtm

echo 'After Exchange'
resim show $account


# Cancel the first offer
echo '>>> Cancel ' 
resim run escrow/cancel_escrow.rtm

echo 'After Cancel'
resim show $account





# Withdraw
echo '>>> Withdraw ' 
resim run escrow/withdraw.rtm

echo 'After Withdraw'
resim show $account




