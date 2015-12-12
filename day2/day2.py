feets = 0
for i in open("day2file", "r").readlines():
    nums = [int(x) for x in i.split("x")]
    areas = sorted([nums[0] * nums[1], nums[1] * nums[2], nums[2] * nums[0]])
    feets += ((2 * areas[0]) +
              (2 * areas[1]) +
              (2 * areas[2]) +
              (areas[0]))


print feets
