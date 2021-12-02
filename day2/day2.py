import re
import sys
from functools import reduce

code_pattern = re.compile("(?P<code>up|down|forward) (?P<amount>[0-9]+)")

def move(starting_pos, line):
    (horiz, depth) = starting_pos
    m = code_pattern.match(line.strip())
    if m:
        code = m.group("code")
        amount = int(m.group("amount"))
        if code == "up":
            depth -= amount
        elif code == "down":
            depth += amount
        else:
            horiz += amount
    return (horiz, depth)

lines = list(sys.stdin.readlines())
ending_location = reduce(move, lines, (0,0))

print(f"Part 1: Ending position H: {ending_location[0]} D: {ending_location[1]}; Result {ending_location[0] * ending_location[1]}")

def move_with_aim(starting_pos, line):
    (horiz, depth, aim) = starting_pos
    m = code_pattern.match(line.strip())
    if m:
        code = m.group("code")
        amount = int(m.group("amount"))
        if code == "up":
            aim -= amount
        elif code == "down":
            aim += amount
        else:
            horiz += amount
            depth += amount * aim
    return (horiz, depth, aim)

ending_location_part2 = reduce(move_with_aim, lines, (0, 0, 0))

print(f"Part 2: Ending position H: {ending_location_part2[0]} D: {ending_location_part2[1]}; Result {ending_location_part2[0] * ending_location_part2[1]}")
