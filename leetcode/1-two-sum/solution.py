class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        comps = []
        for index, num in enumerate(nums):
            if num in comps:
                return [comps.index(num), index]
            comps.append(target - num)
        return []