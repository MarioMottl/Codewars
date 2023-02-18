def create_phone_number(n):
    list_string = list(map(str, n))
    number = f"({''.join(list_string[:3])}) {''.join(list_string[3:6])}-{''.join(list_string[6:])}"
    return number
    
def main():
    create_phone_number([1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    create_phone_number([0, 2, 3, 0, 5, 6, 0, 8, 9, 0])
    
if __name__ == "__main__":
    main()