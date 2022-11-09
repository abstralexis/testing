class NotNumericException(Exception):
    pass

class Numeric:
    """
    Class meant to be able to represent any numeric datatype,
    inspired by the string method .isnumeric. I want to eventually
    make an implementation of this in C.
    """
    def __init__(self, num):
        """
        A generic initialisation. 
        """
        self.num = num
        self.string = None

    def __int__(self) -> int:
        return int(self.num)

    def __float__(self) -> float:
        return float(self.num)

    def __complex__(self) -> complex:
        return complex(self.num)

    def __str__(self) -> str:
        return f"{self.num}"
        
    def __add__(self, numeric):
        return Numeric(self.num + numeric.num)

    @classmethod
    def new(cls, num):
        """
        A constructor that is more explicit about
        what it does.
        """
        return cls(num)

    @classmethod
    def from_int(cls, num: int):
        return cls(num)

    @classmethod
    def from_float(cls, num):
        return cls(num)


    @classmethod
    def from_complex(cls, num: complex):
        return cls(num)

    @classmethod
    def from_str(cls, num_str: str):
        if not num_str.isnumeric():
            raise NotNumericException("str must be numeric to pass to Numeric.from_str()")
        return cls.from_float(float(num_str))