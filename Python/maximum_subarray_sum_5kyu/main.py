from math import inf



def max_sequence(arr):
    max_so_far = -inf -1
    max_ending_here = 0
    
    for i in range(len(arr)):
        max_ending_here = max_ending_here + arr[i]
        
        if max_so_far < max_ending_here:
            max_so_far = max_ending_here
        
        if max_ending_here < 0:
            max_ending_here = 0
    max_so_far = 0 if max_so_far < 0 else max_so_far
    
    print(max_so_far)
    return max_so_far
    
def main():
    max_sequence([-2, -1, -3, -4, -1, -2, -1, -5, -4]) # 0
    max_sequence([7, 4, 11, -11, 39, 36, 10, -6, 37, -10, -32, 44, -26, -34, 43, 43]) # 155
    
if __name__ == "__main__":
    main()