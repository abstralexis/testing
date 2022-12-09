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
                if index > 25:
                    ciphertext += upper[index - len(upper)]
                elif index < 0:
                    ciphertext += upper[index + len(upper)]
    return ciphertext

def decrypt(ciphertext: str, key: int) -> str:
    ciphertext = ciphertext.upper()
    plaintext = ""
    for character in ciphertext:
        if character not in upper:
            plaintext += character
        else:
            index = upper.find(character) - key
            if index < len(upper):
                plaintext += upper[index]
            else:
                if index > 25:
                    plaintext += upper[index - len(upper)]
                elif index < 0:
                    plaintext += upper[index + len(upper)]
    return plaintext

def main():
    print(encrypt("Hello World!", 2))
    print(decrypt(encrypt("Hello World!", 2), 2))

if __name__ == "__main__":
    main()