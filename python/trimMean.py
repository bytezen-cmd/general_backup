from typing import List

class Solution:
    def trimMean(self, arr: List[int]) -> float:
        a = len(arr)
        b = int(a * 5 / 100)
        arr = sorted(arr)
        arr = arr[b:a - b]
        return sum(arr) / len(arr)

a = Solution()
print(a.trimMean([1,2,3,4,5,6,10,7,8,9,11,12,13,14,15,16,17,18,19,20]))