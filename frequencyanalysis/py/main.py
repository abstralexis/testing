from string import ascii_lowercase

def analyse(text: str) -> dict[str, int]:
    lowertext = text.lower()
    frequencies: dict[str, int] = {}
    for letter in ascii_lowercase:
        frequencies[letter] = lowertext.count(letter)
    return frequencies

def descending_sort(dictionary: dict[str, int]) -> dict[str, int]:
    return dict(
        sorted(
            dictionary.items(), key=lambda x: x[1], reverse=True
        )
    )

if __name__ == "__main__":
    with open("text.txt", "r") as text:
        frequencies: dict[str, int] = analyse(text.read())
        print(frequencies)
        
    print("--- Unsorted ---")
    for letter, freq in frequencies.items():
        print(f"{letter} | {freq}")
        
    sorteddict: dict[str, int] = descending_sort(frequencies)    
    
    print("--- Sorted ---")
    for letter, freq in sorteddict.items():
        print(f"{letter} | {freq}")