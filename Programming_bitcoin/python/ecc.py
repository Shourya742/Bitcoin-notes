from finite_element import FiniteElement
from point import Point
from ECC_test import ECCTest

a = FiniteElement(num=0, prime=223)
b = FiniteElement(num=7, prime=223)
x1 = FiniteElement(num=192, prime=223)
y1 = FiniteElement(num=105, prime=223)
x2 = FiniteElement(num=17, prime=223)
y2 = FiniteElement(num=56, prime=223)

p1 = Point(x1, y1, a, b)
p2 = Point(x2, y2, a, b)
# print(p1+p2)

# print(Point(FiniteElement(170, 223), FiniteElement(142, 223), a, b) +
#       Point(FiniteElement(60, 223), FiniteElement(139, 223), a, b))
# print(Point(FiniteElement(47, 223), FiniteElement(71, 223), a, b) +
#       Point(FiniteElement(17, 223), FiniteElement(56, 223), a, b))
# print(Point(FiniteElement(143, 223), FiniteElement(98, 223), a, b) +
#       Point(FiniteElement(76, 223), FiniteElement(66, 223), a, b))

x3 = FiniteElement(47, 223)
y3 = FiniteElement(71, 223)
p = Point(x3, y3, a, b)
for s in range(1, 21):
    result = s*p
    # print(f"{s}*{47,71} = ({result.x.num} {result.y.num})")

gx = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798
gy = 0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8
p = 2**256 - 2**32 - 977
print(gy**2 % p == (gx**3+7) % p)

n = 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141
x = FiniteElement(gx, p)
y = FiniteElement(gy, p)
seven = FiniteElement(7, p)
zero = FiniteElement(0, p)
G = Point(x, y, zero, seven)
# print(n*G)
