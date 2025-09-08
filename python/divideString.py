from typing import List

class Solution:
    def divideString(self, s: str, k: int, fill: str) -> List[str]:
        c = []
        for b in range(0,len(s),k):
            c.append(s[b:b+k])
        if len(c[-1]) != k:
            c[-1] = c[-1] + (k - len(c[-1])) * fill
        return c


a = Solution()
print(a.divideString("abcdefg", 3, "x"))