from stellar_sdk import Account, Asset, Keypair, Network, TransactionBuilder, Server

# Replace with your secret key
root_secret_key = "SB5RWBRSCG4PFUBAPIPBMQWVUAXBMH7JG2KANLWBVOJSSEVFLGIYNRRK"
contract_id = "GAFBM3TQNO6QY55XIU2D4SDV2X4UR2BYFIHLBNQNMXSOPFCDAOXDCWTL"

# Initialize keypair and server
root_keypair = Keypair.from_secret(root_secret_key)
server = Server("https://horizon-testnet.stellar.org")

# Fetch the current sequence number for the root account
root_account = server.load_account(account_id=root_keypair.public_key)

# Build the transaction
transaction = (
    TransactionBuilder(
        source_account=root_account,
        network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,
        base_fee=100,
    )
    .append_payment_op(
        destination=contract_id,
        asset=Asset.native(),
        amount="125.5",
    )
    .append_set_options_op(
        home_domain="overcat.me"
    )
    .set_timeout(30)
    .build()
)

# Sign the transaction
transaction.sign(root_keypair)

# Submit the transaction to the network
response = server.submit_transaction(transaction)
print("Transaction successful!")
print(f"Transaction hash: {response['hash']}")

