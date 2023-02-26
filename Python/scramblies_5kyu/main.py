def scramble(s1: str, s2: str):
    for char in s2:
        if s1.count(char) > 0 and s1.count(char) >= s2.count(char):
            ...
        else:
            return False
    return True

def main():
    print(scramble('rkqodlw', 'world'))
    print(scramble('cedewaraaossoqqyt', 'codewars'))
    print(scramble('katas', 'steak'))

if __name__ == "__main__":
    main()