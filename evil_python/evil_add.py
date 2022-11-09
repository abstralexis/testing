from time import sleep, time

def evil_add(num1: int, num2: int) -> int:
    """
    adds two positive integers
    """
    start = time()
    sleep(num1)
    sleep(num2)
    return int(time() - start)

if __name__ == "__main__":
    print(evil_add(1, 2))