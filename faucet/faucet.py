#!/usr/bin/env python3
import os
import random
import discord
import hashlib
import base58
from discord.ext import commands
from substrateinterface import SubstrateInterface, Keypair, KeypairType
from substrateinterface.exceptions import SubstrateRequestException
from substrateinterface.utils.ss58 import ss58_encode, is_valid_ss58_address
from scalecodec.type_registry import load_type_registry_file
from ethereum.utils import check_checksum

# Your Discord bot token
TOKEN = '<discord_application_id>'
bot = commands.Bot(command_prefix='!')
# keypair = Keypair.create_from_mnemonic(faucet_mnemonic,crypto_type=KeypairType.SR25519)
keypair3 = Keypair.create_from_uri('//Alice')
print(keypair3)
# substrate RPC node
node_rpc = "ws://127.0.0.1:9944"

custom_type_registry = load_type_registry_file("my-custom-types.json")

@bot.command(name='send', help='Send token from faucet')
async def nine_nine(ctx, arg):
    print(ctx.channel.type)
    finalAddress=arg
    validAddress=True
    if (ctx.channel.type == discord.ChannelType.private):
        # Forbid DM in discord
        await ctx.send("Hey ICE-Dev, you can't send me private messages!, Request tokens using command(!send <address>) in the Guild")
    else:
        if(is_valid_ss58_address(arg)):
            finalAddress=arg
            print("Requested for substrate address:"+finalAddress)          
        elif (check_checksum(arg)):                        
            addressBytes = bytes.fromhex(arg[2:])
            prefixBytes = bytes("evm:", 'utf-8')
            convertBytes = prefixBytes+addressBytes
            uint8array=list(convertBytes)
            hash = hashlib.blake2b(digest_size=32)
            hash.update(convertBytes)
            ss8Format=ss58_encode(hash.digest().hex(),42)
            finalAddress=ss8Format
            print("Requested for ethereum address:"+finalAddress) 
        else:
            validAddress=False
            await ctx.send("Invalid Address format, Please request only for ss58/ethereum addresses")
        
        if validAddress:        
            substrate = SubstrateInterface(
                url=node_rpc,
                ss58_format=42,
                type_registry_preset='substrate-node-template',
                type_registry=custom_type_registry
            )
            call = substrate.compose_call(
            call_module='Balances',
            call_function='transfer',
            call_params={
                'dest': finalAddress,
                'value': 10 * 10**18
                }
            )   
            reply = ""
            extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair3)
            reply = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
            await ctx.send(ctx.author.mention + " Hey ICE-Dev, we sent you 10 ICY token, it has no real value can be used only on the testnet!,\n"+"Mapped Address: "+finalAddress+"\n Txn Hash: " +  reply['extrinsic_hash'])

bot.run(TOKEN)