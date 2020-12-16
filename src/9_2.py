with open("data/9.csv", "r") as f:
    DATA = f.readlines()


index = 0
watchlist = []
NUMBERS = [int(line.strip()) for line in DATA]
TARGET = 10884537

while index < len(NUMBERS):
    num = NUMBERS[index]
    for r in watchlist:
        r.append(num)
    watchlist.append([num])

    watchlist = list(filter(lambda x: sum(x) <= TARGET, watchlist))

    sums_to_target = list(
        filter(lambda x: sum(x) == TARGET and len(x) > 1, watchlist))
    if len(sums_to_target) > 0:
        for res in sums_to_target:
            print("Yeah" + str(res) + " " + str(sum(res)) +
                  " " + str(min(res) + max(res)))

    index = index + 1
