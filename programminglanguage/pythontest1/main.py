"""
This is some really basic (bad) interpreter that takes a
very specific string, tokenises, lexes, then parses the 
string and it can handle making a global int variable.

ex.
var num = 100
"""

from dataclasses import dataclass
from enum import Enum
from string import ascii_letters

class TokenType(Enum):
    Identifier = 1
    Number = 2
    String = 3
    Function = 4
    Assignment = 5
    Boolean = 6
    EOF = 7
    Type = 8
    Var = 9
    Blank = 10
    Operator = 11
    
@dataclass
class Keyword:
    keywords = (
        
    )

@dataclass
class Token:
    token_type: TokenType
    contents: str
    
@dataclass
class Blank(Token):
    token_type: TokenType = TokenType.Blank
    contents: str = ""
    

class Lexer:
    def __init__(self, text: str):
        self.text = text
        self.tokenised = self.tokenise()
        self.lexed = self.lex()
        
    def tokenise(self) -> list[str]:
        # Split at whitespace for now
        return self.text.split(" ")

    def lex(self) -> list[Token]:
        lexed: list[Token] = []
        for i, token in enumerate(self.tokenised):
            prev: Token = Blank() if len(lexed) < 1 else lexed[-1]

            # Keywords
            if token == "var":
                lexed.append(Token(TokenType.Var, token))

            # Other tokens
            elif prev is not Blank:
                if prev.token_type == TokenType.Var:
                    lexed.append(Token(TokenType.Identifier, token))
                elif prev.token_type == TokenType.Identifier:
                    if token == "=":
                        lexed.append(Token(TokenType.Assignment, token))
                elif prev.token_type == TokenType.Assignment:
                    if token in "01234567890":
                        lexed.append(Token(TokenType.Number, token))
                    
        return lexed
    

class Parser:
    def __init__(self, tokens: list[Token]):
        self.tokens = tokens
        self.parse()
        
    def parse(self):
        for i, token in enumerate(self.tokens):
            nxt = Blank() if i == len(self.tokens) - 1 else self.tokens[i + 1]
            if token.token_type == TokenType.Var:
                try:
                    if self.tokens[i+1].token_type == TokenType.Identifier:
                        if self.tokens[i+2].token_type == TokenType.Assignment:
                            if self.tokens[i+3].token_type == TokenType.Number:
                                globals()[self.tokens[i+1].contents] = self.tokens[i+3]
                except:
                    pass


if __name__ == "__main__":
    text = "var hello = 1 var num = 2"
    lexer = Lexer(text)
    tokens = lexer.tokenise()
    lexed = lexer.lex()
    parser = Parser(lexed)
    print(tokens)
    print(lexed)
    print(globals())
    print(globals()["hello"].contents)
    print(globals()["num"].contents)
    