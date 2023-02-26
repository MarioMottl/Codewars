def solution(s):
    newString: str = ""
    for char in s:
        if char.isupper():
            newString += " "
        newString += char
    
    return newString
        

def main():
    print(solution("breakCamelCase"))
    print(solution("camelCase"))
    print(solution("helloWorld"))
    print(solution("identifier"))
    
    

if __name__ == "__main__":
    main()