with open("data/9.csv", "r") as f:
    DATA = f.readlines()

NUMS = [int(line.strip()) for line in DATA]
PREAMBLE_LENGTH = 25

INDEX = range(PREAMBLE_LENGTH, len(NUMS))
INDICES = [(i, list(range(i - PREAMBLE_LENGTH, i))) for i in INDEX]
ACTUAL_NUMBERS = [(NUMS[target], list(map(lambda a: NUMS[a], r)))
                  for (target, r) in INDICES]


def crossproduct(l1, l2):
    return list((a, b) for a in l1 for b in l2)


COMBINATIONS = [(target, crossproduct(nums, nums))
                for (target, nums) in ACTUAL_NUMBERS]

COMPUTED = [
    (target, list(map(lambda a: a[0]+a[1], candidates))) for (target, candidates) in COMBINATIONS
]

COUNTED = [
    (target, len(list(filter(lambda c: c == target, candidates)))) for (target, candidates) in COMPUTED
]

COUNTED

res = list(filter(lambda x: x[1] == 0, COUNTED))
res[0]
