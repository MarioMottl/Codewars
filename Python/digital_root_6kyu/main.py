"""
16  -->  1 + 6 = 7
942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2
"""

def digital_root(n: int):
    numbers = [int(x) for x in str(n)]
    print(f"digital_root({numbers})")
    root = sum(numbers)
    if root > 9:
        root = digital_root(root)
    print(f"ROOT: {root}")
    return root

print(digital_root(942))