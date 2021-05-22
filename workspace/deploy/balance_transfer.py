from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException

# import logging
# logging.basicConfig(level=logging.DEBUG)

try:
    substrate = SubstrateInterface(
        url="ws://127.0.0.1:9944",
        # use_remote_preset=True
        ss58_format=42,
        type_registry_preset='substrate-node-template',
    )
    # substrate.reload_type_registry()  # +
except ConnectionRefusedError:
    print("⚠️ No local Substrate node running, try running 'start_local_substrate_node.sh' first")
    exit()

keypair = Keypair.create_from_uri('//Alice')

account_info = substrate.query('System', 'Account', params=[keypair.ss58_address])


print('Account info', account_info.value)

call = substrate.compose_call(
    call_module='Balances',
    call_function='transfer',
    call_params={
        'dest': '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
        'value': 1 * 10**15
    }
)

# Get payment info
payment_info = substrate.get_payment_info(call=call, keypair=keypair)

print("Payment info: ", payment_info)

extrinsic = substrate.create_signed_extrinsic(
    call=call,
    keypair=keypair,
    era={'period': 64}
)


try:
    receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

    print('Extrinsic "{}" included in block "{}"'.format(
        receipt.extrinsic_hash, receipt.block_hash
    ))

    if receipt.is_success:
        print('✅ Success, triggered events:')
        for event in receipt.triggered_events:
            print(f'* {event.value}')

    else:
        print('⚠️ Extrinsic Failed: ', receipt.error_message)


except SubstrateRequestException as e:
    print("Failed to send: {}".format(e))
