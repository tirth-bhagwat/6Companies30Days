# https://www.geeksforgeeks.org/problems/run-length-encoding/1/


def encode(arr):
    res = ""
    prev = arr[0]
    count = 1

    for ch in arr[1:]:
        if ch == prev:
            count += 1
            continue

        res += prev
        res += str(count)

        prev = ch
        count = 1

    res += prev
    res += str(count)

    return res
