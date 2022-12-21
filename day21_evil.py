import sys

monkeys = {}

for line in sys.stdin:
    line = line.strip()
    name = line[:4]
    monkeys[name] = line.replace(": ", " = ")

lines = []
already_used = set()

def toposort(name):
    if name in already_used:
        return
    already_used.add(name)

    monkey = monkeys[name]
    if len(monkey) < 15:
        lines.append(monkey)
    else:
        toposort(monkey[7:11])
        toposort(monkey[14:])
        lines.append(monkey.replace("/", "//"))

toposort("root")

script = "\n".join(lines)
print(script)
exec(script)
print(root)