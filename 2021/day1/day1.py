import sys
from functools import reduce

def increaser(accum, val):
    (counter, previous) = accum
    if val > previous:
        return (counter + 1, val)
    return (counter, val)

data = [int(x) for x in (x.strip() for x in sys.stdin.readlines()) if x != '']

print("Puzzle 1:", reduce(increaser, data[1:], (0, data[0]))[0])

from dataclasses import dataclass
@dataclass
class Accumulator:
    counter: int
    sum: int
    spot: int
    value: int

def windowing(accum, val):
    counter = accum.counter
    previous_sum = accum.sum
    spot0 = accum.spot
    spot1 = accum.value
    new_sum = spot0 + spot1 + val
    if new_sum > previous_sum:
        new_counter = counter + 1
    else:
        new_counter = counter
    return Accumulator(new_counter, new_sum, spot1, val)

print("Puzzle 2:", reduce(windowing, data[3:], Accumulator(0, sum(data[0:3]), data[1], data[2]))[0])
