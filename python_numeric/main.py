from numeric import Numeric
from cmath import sqrt

"""
The main use of Numerics is to be able to say the argument to
a function that can use all types of numbers is a generic number type.
"""
def numericmath(x: Numeric, y: Numeric):
    return x + y

if __name__ == "__main__":
    a = Numeric(5)
    b = Numeric.new(5)
    c = Numeric.from_float(5.0)
    d = Numeric.from_int(5)
    e = Numeric.from_complex(sqrt(-1))
    f = Numeric.from_str("5")
    # g = Numeric.from_str("Hello")     # returns NotNumericException

    print(f"{a} {b} {c} {d} {e} {f}")
    floata = float(a)
    intc = int(c)
    strd = str(d)

    print(numericmath(a, b))