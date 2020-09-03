class Solution:
    def reverseString(self, s: str) -> str:
        length = len(s)
        result = [None] * length
        start, end = 0, length - 1
        while start <= end:
            result[start], result[end] = s[end], s[start]
            start, end = start + 1, end - 1
        return ''.join(result)
    
    def reverse(self, x: int) -> int:
        result = int(self.reverseString(str(abs(x))))
        if result > 2147483647:
            return 0
        return result if x > 0 else -result