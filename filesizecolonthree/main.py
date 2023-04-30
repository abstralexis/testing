# Assumes no metadata, just raw text, UTF-8

def colon3(size_bytes: int) -> None:
    with open("uwu.txt", "w") as uwu:
        uwu.write(":")
        for _ in range(size_bytes - 1):
            uwu.write("3")
            
if __name__ == "__main__":
    colon3(50*(1024**2))