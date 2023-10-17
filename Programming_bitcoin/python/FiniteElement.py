class FiniteElement:
    def __init__(self, num, prime):
        if num >= prime or num < 0:
            error = f"Num {self.num} not in field range 0 to {self.prime}"
            raise ValueError(error)
        self.num = num
        self.prime = prime

    def __repr__(self):
        return f"FieldElement_{self.prime}({self.num})"

    def __eq__(self, other):
        if other is None:
            return False
        return self.num == other.num and self.prime == other.prime

    def __ne__(self, other):
        if other is None:
            return True
        return not (self == other)

    def __add__(self, other):
        if self.prime != other.prime:
            raise TypeError('Cannot add two numbers in different Fields')
        return self.__class__((self.num+other.num) % self.prime, self.prime)

    def __sub__(self, other):
        if (self.prime != other.prime):
            raise TypeError("Cannot sub two numbers with different Fields")
        return self.__class__((self.num-other.num + self.prime) % self.prime, self.prime)

    def __mul__(self, other):
        if (self.prime != other.prime):
            raise TypeError(
                "Cannot multiple to numbers belonging to different fields")
        return self.__class__((self.num*other.num) % self.prime, self.prime)

    def __pow__(self, exponent):
        return self.__class__(pow(self.num, exponent % (self.prime-1), self.prime))

    def __truediv__(self, other):
        if self.prime != other.prime:
            raise TypeError('Cannot divide two numbers with different field')
        return self.__class__((self.num*pow(other.num, self.prime-2, self.prime) % self.prime, self.prime))


if __name__ == "__main__":
    a = FiniteElement(7, 13)
    b = FiniteElement(12, 13)

    print(a != b)
    print(a == a)
    print(a)
    print(b)

    c = FiniteElement(6, 13)
    print(a+b == c)

    prime = 19
    for k in (1, 3, 7, 13, 18):
        print([k*i % prime for i in range(prime)])
    for k in (1, 3, 7, 13, 18):
        print(sorted([k*i % prime for i in range(prime)]))

    a = FiniteElement(3, 13)
    b = FiniteElement(12, 13)
    c = FiniteElement(10, 13)

    print(a*b == c)

    for prime in (7, 11, 17, 31):
        print([pow(i, prime-1, prime) for i in range(1, prime)])

    a = FiniteElement(7, 13)
    b = FiniteElement(8, 13)
    print(a**-3)
    print(a**-3 == b)
