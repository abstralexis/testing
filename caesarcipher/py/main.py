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

def brute_force(ciphertext: str) -> list[str]:
    decryptions: list[str] = []
    for i in range(26):
        decryptions.append(decrypt(ciphertext, i))
    return decryptions    

def main():
    print(encrypt("THE SUN HAS GOT ITS HAT ON", 7))
    print(decrypt("MNU MNU MTTWFD", 5))
    print(decrypt("GUVF VF ZL FRPERG ZRFFNTR", 13))
    print(decrypt(encrypt("Hello World!", 2), 2))
    print(brute_force("MNU MNU MTTWFD"))

if __name__ == "__main__":
    main()