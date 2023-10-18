class Signature:
    def __init__(self, r, s):
        self.r = r
        self.s = s

    def __repr__(self):
        return f"Signature({self.r},{self.s})"
