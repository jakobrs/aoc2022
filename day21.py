import sys

monkeys = {}

# monkeys[name] = 32
# monkeys[name] = "abcd + defg"

for line in sys.stdin:
    line = line.strip()
    name = line[:4]
    if len(line) < 15:
        monkeys[name] = int(line[6:])
    else:
        monkeys[name] = line[6:]

def evaluate(name):
    monkey = monkeys[name]
    if isinstance(monkey, int):
        return monkey
    else:
        dep1 = evaluate(monkey[:4])
        dep2 = evaluate(monkey[7:])
        op = monkey[5]

        if op == "+":
            res = dep1 + dep2
        elif op == "-":
            res = dep1 - dep2
        elif op == "*":
            res = dep1 * dep2
        elif op == "/":
            res = dep1 // dep2

        monkeys[name] = res
        return res

print(evaluate("root"))