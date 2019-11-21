#!/usr/bin/env python3
ROWS = 4
COLS = 4
LAYERS = 4

def make_vertices():
    for i in range(2 * ROWS + 1):
        for j in range(2 * COLS + 1):
            for k in range(2 * LAYERS + 1):
                x = i * 0.5
                y = j * 0.5
                z = k * 0.5
                yield f'v {x} {y} {z}\n'

with open("celtic_grid.obj", 'w') as f: 
    for line in make_vertices():
        f.write(line)

