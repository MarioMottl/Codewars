def find_it(seq: list):
    for i in seq:
        occurences = seq.count(i)
        if occurences % 2 != 0:
            return i
        

def main():
    print(find_it([20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5]))
    print(find_it([1,1,2,-2,5,2,4,4,-1,-2,5]))
    print(find_it([1,1,1,1,1,1,10,1,1,1,1]))

if __name__ == "__main__":
    main()