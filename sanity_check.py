import siphasher, siphash, siphash24

h0 = siphasher.SipHash24(0, 0)
h1 = siphash24.siphash24()
h2 = siphash.SipHash24(secret=b'\x00'*16)
h0.update(b'hello')
h1.update(b'hello')
h2.update(b'hello')

# siphasher returns `str` from hexdigest, like `hashlib` but unlike the other 2 pure-Python hash impls
h0_res = (h0.digest(), h0.hexdigest().encode('utf-8'))
h1_res = (h1.digest(), h1.hexdigest())
h2_res = (h2.digest(), h2.hexdigest())
assert h0_res == h1_res
print('siphasher\t', h0_res)
print('siphash24\t', h1_res)
print('siphash  \t', h2_res)
