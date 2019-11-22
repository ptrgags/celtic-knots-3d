#!/usr/bin/env python3

def get_bounds(num_nodes):
    (N, M, P) = num_nodes
    x_bounds = (0.0, 2.0 * N)
    y_bounds = (0.0, 2.0 * M)
    z_bounds = (0.0, 2.0 * P)
    return (x_bounds, y_bounds, z_bounds)

def find_crossings(num_nodes):
    (N, M, P) = num_nodes
    for i in range(1, 2 * N):
        for j in range(1, 2 * M):
            for k in range(1, 2 * P):
                xz_odd = (i + k) % 2 == 1
                yz_odd = (j + k) % 2 == 1
                xy_even = (i + j) % 2 == 0
                if xz_odd and yz_odd and xy_even:
                    yield (i, j, k)

def cross_tile(center):
    (x, y, z) = center
    yield ((x - 0.5, y - 0.5, z - 0.5), (x + 0.5, y + 0.5, z + 0.5))
    yield ((x + 0.5, y - 0.5, z - 0.5), (x - 0.5, y + 0.5, z + 0.5))
    yield ((x + 0.5, y + 0.5, z - 0.5), (x - 0.5, y - 0.5, z + 0.5))
    yield ((x - 0.5, y + 0.5, z - 0.5), (x + 0.5, y - 0.5, z + 0.5))

def all_cross_tiles(num_nodes):
    for crossing in find_crossings(num_nodes):
        yield from cross_tile(crossing)

class SegmentWriter:
    def __init__(self):
        self.vertices = []
        self.segments = []

    def add_segments(self, vertex_pairs):
        start_index = len(self.vertices)

        for i, (a, b) in enumerate(vertex_pairs):
            self.vertices.append(a)
            self.vertices.append(b)
            self.segments.append([
                start_index + (2 * i) + 1, 
                start_index + (2 * i + 1) + 1
            ])

    def write(self, fname):
        with open(fname, 'w') as f:
            for x, y, z in self.vertices:
                f.write(f'v {x} {y} {z}\n')

            for segment in self.segments:
                indices_str = ' '.join(str(i) for i in segment)
                f.write(f'l {indices_str}\n')

if __name__ == '__main__':
    num_nodes = (4, 2, 3)

    writer = SegmentWriter()
    writer.add_segments(all_cross_tiles(num_nodes))
    writer.write('celtic_tiles.obj')
