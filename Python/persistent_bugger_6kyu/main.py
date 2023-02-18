def persistence(n):

    run = 0
    
    def inner(n):
        nonlocal run
        print(n)
        numbers = [int(x) for x in str(n)]
        m = 1
        for x in numbers:
            m = m * x
        root =  m
        print(f"Mul Result: {root}")
        if root > 9:
            run = run + 1
            root = inner(root) 
        
    inner(n)
    if run != 0 or n > 9:
        run += 1
        
    print(f"RUN: {run}")
    return run

def main():
    persistence(39)
    persistence(999)
    persistence(7)

if __name__ == "__main__":
    main()