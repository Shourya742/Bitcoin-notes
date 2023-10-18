class Point:
    def __init__(self, x, y, a, b):
        self.a = a
        self.b = b
        self.x = x
        self.y = y
        if self.x is None and self.y is None:
            return
        print(self.y)
        print(self.x)
        print(self.a)
        print(self.b)
        if self.y**2 != self.x**3+a*self.x+b:
            raise ValueError(f"({self.x} {self.y}) is not on the curve")

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.a == other.a and self.b == other.b

    def __ne__(self, other):
        return not (self == other)

    def __add__(self, other):
        if self.a != other.a or self.b != self.b:
            raise TypeError(f"Points {self}, {other} are not the same curve")

        if self.x is None:
            return other
        if other.x is None:
            return self
        if other.x == self.x and other.y != self.y:
            return self.__class__(None, None, self.a, self.b)
        if other.x != self.x:
            slope = (self.y-other.y)/(self.x-other.x)
            x3 = slope*slope-self.x-other.x
            y3 = slope*(self.x-x3)-self.y
            return self.__class__(x3, y3, self.a, self.b)
        if other == self and self.y == 0*self.x:
            return self.__class__(None, None, self.a, self.b)
        if other.x == self.x and self.y == other.y:
            slope = (3*self.x*self.x+self.a)/(2*self.y)
            x3 = slope*slope - 2*self.x
            y3 = slope*(self.x-x3)-self.y
            return self.__class__(x3, y3, self.a, self.b)

    def __repr__(self):
        if self.x != None:
            return f"Point({self.x},{self.y})_{self.a}_{self.b}"
        return f"Point(infinity)"

    def __rmul__(self, coefficient):
        # Binary Expansion
        current = self
        result = self.__class__(None, None, self.a, self.b)
        while coefficient:
            if coefficient & 1:
                result += current
            current += current
            coefficient >>= 1
        return result
        # result = self.__class__(None, None, self.a, self.b)
        # for _ in range(coefficient):
        #     result += self
        # return result


def on_curve(x, y):
    return y**2 == x**3+5*x+7


if __name__ == "__main__":
    p1 = Point(-1, -1, 5, 7)
    # Validating Error
    # p2 = Point(-1, -2, 5, 7)
    print(on_curve(2, 4))
    print(on_curve(-1, -1))
    print(on_curve(18, 77))
    print(on_curve(5, 7))
    p1 = Point(-1, -1, 5, 7)
    p2 = Point(-1, 1, 5, 7)
    inf = Point(None, None, 5, 7)
    print(p1+inf)
    print(p1+p2)
    p1 = Point(2, 5, 5, 7)
    p2 = Point(-1, -1, 5, 7)
    print(p1+p2)
    p1 = Point(-1, -1, 5, 7)
    p2 = Point(-1, -1, 5, 7)
    print(p1+p2)
