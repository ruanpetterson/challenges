class Solution:
    def reverseString(self, s: List[str]) -> None:
        size = len(s)
        start, end = 0, size - 1
        while (start <= end):
            s[start], s[end] = s[end], s[start]
            start, end = start + 1, end - 1