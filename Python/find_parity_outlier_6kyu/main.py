def find_outlier(integers):
    odd = []
    even = []
    
    for i in integers:
        if i % 2 == 0:
            even.append(i)
        else:
            odd.append(i)
            
    return odd[0] if len(odd) < len(even) else even[0]

def main():
    print(find_outlier([2, 4, 6, 8, 10, 3]))
    print(find_outlier([2, 4, 0, 100, 4, 11, 2602, 36]))
    print(find_outlier([160, 3, 1719, 19, 11, 13, -21]))

if __name__ == "__main__":
    main()