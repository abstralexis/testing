class Add:
    def __init__(self, a, b):
        self.a = a
        self.b = b

    def add(self):
        return self._add()

    def _add(self):
        return self.a + self.b

    def __float__(self):
        return self.a - self.b

    @staticmethod
    def print():
        print("i just needed a commit for today bruh")

def call_main():
    main()

def main():
    add = Add(0.1, 0.2)
    added = add.add()
    print(float(add), added)
    add.print()
    assert float(add) == added

if __name__ == "__main__":
    call_main()