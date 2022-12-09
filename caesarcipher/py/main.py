import string

upper = string.ascii_uppercase

def encrypt(plaintext: str, key: int) -> str:
    plaintext = plaintext.upper()
    ciphertext = ""
    for character in plaintext:
        if character not in upper:
            ciphertext += character
        else:
            index = upper.find(character) + key
            if index < len(upper):
                ciphertext += upper[index]
            else:
                ciphertext += upper[index - len(upper)]
    return ciphertext

def main():
    print(encrypt("Hello World!", 2))

if __name__ == "__main__":
    main()