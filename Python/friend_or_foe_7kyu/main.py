def friend(x):
    great_filter = lambda a : True if len(a) == 4 and not a.isnumeric() else False
    new_x = list(filter(great_filter, x))
    print(new_x)
    return new_x


def main():
    friend(["Ryan", "Kieran", "Mark",])
    friend(['Ryan', '123', '4'])

if __name__ == "__main__":
    main()