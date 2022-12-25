"""
This is the python code for one of my Project Euler 
solutions. It illustrates functional programming in 
python nicely, although not with all the functions
that python has.
"""

def sum_square_upto(n: int) -> int:
    return sum(map(lambda x: x**2, range(1, n+1)))

def square_sum_upto(n: int) -> int:
    return sum(range(1, n+1))**2

def main():
    n = 100
    print(square_sum_upto(n) - sum_square_upto(n))

if __name__ == "__main__":
    main()