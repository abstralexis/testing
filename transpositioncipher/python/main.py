# A simple transposition cipher

import numpy as np
from functools import reduce

class TranspositionCipher:
    def __init__(self, plaintext: str, key: int):
        self.plaintext = plaintext
        self.key = key
    
    def encrypted(self) -> str:
        return self.__ciphered()
    
    def decrypted(self) -> str:
        return TranspositionCipher(
            self.__ciphered(), 
            len(self.__ciphered())//self.key
        ).__ciphered()
        
    def __table(self) -> list[list[str]]:
        """
        Transposes self.plaintext to an array of arrays with
        length self.key of 1 char string slices of self.plaintext
        """
        return list(map(
            lambda x: 
                # Make sure each item has key length by adding whitespace
                list(x) if len(x) == self.key 
                else list((x + (" "*self.key))[:self.key]),
            [
            self.plaintext[i: i + self.key]
            for i in range(0, len(self.plaintext), self.key)
            ]
        ))
        
    def __transposed(self) -> list[list[str]]:
        """
        Transposes the table
        """
        return np.array(self.__table()).T.tolist()
    
    def __ciphered(self) -> str:
        """
        Returns the ciphertext from self.plaintext according to self.key.
        """
        return "".join(np.array(self.__transposed()).flatten().tolist())
        
if __name__ == "__main__":
    tc = TranspositionCipher("Hello World!", 5)
    
    print(tc.encrypted())
    print(tc.decrypted())