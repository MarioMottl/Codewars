def duplicate_count(text):
    duplicate_counter = 0
    counted_chars = []
    text = text.lower()
    for char in text:
        if text.count(char) > 1 and char not in counted_chars:
            duplicate_counter += 1
            counted_chars.append(char)
    return duplicate_counter

def main():
    print(duplicate_count("abcde"))
    print(duplicate_count("abcdeaa"))
    print(duplicate_count("abcdeaB"))

if __name__ == "__main__":
    main()
