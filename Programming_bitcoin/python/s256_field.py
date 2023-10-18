from finite_element import FiniteElement
from point import Point
from helper import hash256
P = 2**256 - 2**32 - 977
A = 0
B = 7
N = 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141


class S256Field(FiniteElement):

    def __init__(self, num, prime=None):
        super().__init__(num, prime=P)

    def __repr__(self):
        return f"{self.num}".zfill(64)


class S256Point(Point):
    def __init__(self, x, y, a=None, b=None):
        a, b = S256Field(A), S256Field(B)
        if type(x) == int:
            super().__init__(x=S256Field(x), y=S256Field(y), a=a, b=b)
        else:
            super().__init__(x=x, y=y, a=a, b=b)

    def __rmul__(self, coefficient):
        coefficient = coefficient % N
        return super().__rmul__(coefficient)

    def __repr__(self):
        if self.x is None:
            return "S256Point(infinity)"
        else:
            return f"S256Point({self.x},{self.y})"

    def verify(self, z, sig):
        s_inv = pow(sig.s, N-2, N)
        u = z*s_inv % N
        v = sig.r*s_inv % N
        total = u*G + v*self
        return total.x.num == sig.r


G = S256Point(0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798,
              0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8)

if __name__ == "__main__":

    # print(N*G)
    # z = 0xbc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423
    # r = 0x37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6
    # s = 0x8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec
    # px = 0x04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574
    # py = 0x82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4
    # point = S256Point(px, py)
    # s_inv = pow(s, N-2, N)
    # u = z*s_inv % N
    # v = r*s_inv % N
    # print((u*G+v*point).x.num == r)

    # Signing a message with your private key
    # e = int.from_bytes(hash256(b'my secret'), 'big')
    # z = int.from_bytes(hash256(b'my message'), 'big')
    # k = 1234567890
    # r = (k*G).x.num
    # k_inv = pow(k, N-2, N)
    # s = (z+r*e)*k_inv % N
    # point = e*G

    e = 12345
    z = int.from_bytes(hash256('Programming bitcoin!'), 'big')
    k = 1234567890
    r = (k*G).x.num
    k_inv = pow(k, N-2, N)
    s = (z+r*e)*k_inv % N
    print(e*G)
    print(hex(z))
    print(hex(r))
    print(hex(s))
