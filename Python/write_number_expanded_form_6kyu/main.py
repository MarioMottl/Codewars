def expanded_from(n: int):
    number_string = str(n)
    number_length = len(number_string) - 1

    result = ""
    for num in number_string:
        if num != "0":
            result += num
            if number_length > 0:
                    result += "0" * number_length
                    result += " + "
        number_length -= 1

    return result

def main():
    print(expanded_from(12))
    print(expanded_from(42))
    print(expanded_from(70304))

if __name__ == "__main__":
    main()
