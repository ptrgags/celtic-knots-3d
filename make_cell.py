#!/usr/bin/env python3

def dense_grid(subdivisions):
    GRID_SIZE = 0.5
    min_val = -subdivisions // 2
    max_val = subdivisions // 2 + 1
    scale = subdivisions / (2.0 * GRID_SIZE)
    for i in range(min_val, max_val):
        x = i / scale
        for j in range(min_val, max_val):
            y = j / scale
            for k in range(min_val, max_val):
                z = k / scale
                yield (x, y, z)

def write_grid(fname, vertices):
    with open(fname, 'w') as f:
        for x, y, z in vertices:
            f.write(f'v {x} {y} {z}\n')


if __name__ == '__main__':
    write_grid('cell_grid_0.25.obj', dense_grid(4))
    write_grid('cell_grid_0.125.obj', dense_grid(8))
