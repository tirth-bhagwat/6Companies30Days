# https://www.geeksforgeeks.org/problems/find-missing-and-repeating2512/1/


def findTwoElement(arr: list[int], n):
    arr.sort()
    rep = 0
    missing = 0
    for prev, curr in zip([0] + arr, arr):
        if prev == curr:
            rep = prev
            if missing != 0:
                break
            continue

        if prev + 1 != curr:
            missing = prev + 1
            if rep != 0:
                break

    if missing == 0:
        missing = n

    return [rep, missing]


def main():
    #     2
    # 2 2

    # Test case 1
    arr = [2, 2]
    n = 2
    result = findTwoElement(arr, n)
    print(result)  # Expected output: [2, 1]

    # # Test case 4
    arr = [
        13,
        33,
        43,
        16,
        25,
        19,
        23,
        31,
        29,
        35,
        10,
        2,
        32,
        11,
        47,
        15,
        34,
        46,
        30,
        26,
        41,
        18,
        5,
        17,
        37,
        39,
        6,
        4,
        20,
        27,
        9,
        3,
        8,
        40,
        24,
        44,
        14,
        36,
        7,
        38,
        12,
        1,
        42,
        12,
        28,
        22,
        45,
    ]
    n = 47
    result = findTwoElement(arr, n)
    print(result)  # Expected output: [12, 21]


if __name__ == "__main__":
    main()
