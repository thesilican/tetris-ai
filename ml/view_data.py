import json

with open("data/test.json") as f:
    test = json.load(f)

for i, case in enumerate(test):
    res = ""
    res += f"Case {i}\n"
    for i in range(230, -10, -10):
        row = case["input"][i : i + 10]
        for x in row:
            res += "[]" if x == 1 else "  "
        res += "\n"
    res += f"Label: {case['label']}"
    print("".join(res))
    input()
