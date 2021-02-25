# Lexical Analysis

Classifies substrings according to the role and communicaties the classification to the parser.


STRING Input -> Lexer Token(<Class, String>) -> Parser

## Tokenization

Splits the input up into tokens

## Token Classification

Classifies each token (Keywords, symbols, etc):

IDENT: 

strings of letters or digits

Integer:

a non empty string of digits

Keywords:

set of words special to the language

Whitespace:

Non empty blanks
