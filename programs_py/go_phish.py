# go_phish
# Built with Seahorse v0.1.6

from seahorse.prelude import *
import hashlib
import validators

declare_id('DW5y8reFZuDN1DfxDnpeRqFXrusBcEJxErB8Mv4Lc4Vj')

class Phish(Account):
    owner: Pubkey
    n: u64

@instruction
def init(owner: Signer, phish: Empty[Phish]):
    phish = phish.init(payer = owner, seeds = ['randomness', owner])
    phish.n = 0
    phish.owner = owner.key()


# def verify_url(url):
#     if "~" not in url:
#         return True
#     else:
#         return False

@instruction
def go_phish(owner: Signer, phish: Phish, url: String):
    print(f"{url}~{phish.key()}")
    
