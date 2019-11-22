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

def x_cap(center):
    (x, y, z) = center
    yield((x, y - 0.5, z - 0.5), (x, y + 0.5, z + 0.5))
    yield((x, y + 0.5, z - 0.5), (x, y - 0.5, z + 0.5))

def y_cap(center):
    (x, y, z) = center
    yield((x - 0.5, y, z - 0.5), (x + 0.5, y, z + 0.5))
    yield((x + 0.5, y, z - 0.5), (x - 0.5, y, z + 0.5))

def z_cap(center):
    (x, y, z) = center
    yield((x - 0.5, y - 0.5, z), (x + 0.5, y + 0.5, z))
    yield((x + 0.5, y - 0.5, z), (x - 0.5, y + 0.5, z))

def find_x_cap_centers(num_nodes):
    (N, M, P) = num_nodes
    for i in [0, 2 * N]:
        clamped_i = i + 0.5 if i == 0 else i - 0.5
        for j in range(1, 2 * M):
            for k in range(1, 2 * P):
                xz_odd = (i + k) % 2 == 1
                yz_odd = (j + k) % 2 == 1
                xy_even = (i + j) % 2 == 0
                if xz_odd and yz_odd and xy_even:
                    yield (clamped_i, j, k)

def find_y_cap_centers(num_nodes):
    (N, M, P) = num_nodes
    for j in [0, 2 * M]:
        clamped_j = j + 0.5 if j == 0 else j - 0.5
        for i in range(1, 2 * N):
            for k in range(1, 2 * P):
                xz_odd = (i + k) % 2 == 1
                yz_odd = (j + k) % 2 == 1
                xy_even = (i + j) % 2 == 0
                if xz_odd and yz_odd and xy_even:
                    yield (i, clamped_j, k)

def find_z_cap_centers(num_nodes):
    (N, M, P) = num_nodes
    for k in [0, 2 * P]:
        clamped_k = k + 0.5 if k == 0 else k - 0.5
        for j in range(1, 2 * M):
            for i in range(1, 2 * N):
                xz_odd = (i + k) % 2 == 1
                yz_odd = (j + k) % 2 == 1
                xy_even = (i + j) % 2 == 0
                if xz_odd and yz_odd and xy_even:
                    yield (i, j, clamped_k)

def all_2d_caps(num_nodes):
    for pos in find_x_cap_centers(num_nodes):
        yield from x_cap(pos)

    for pos in find_y_cap_centers(num_nodes):
        yield from y_cap(pos)

    for pos in find_z_cap_centers(num_nodes):
        yield from z_cap(pos)

def all_edge_caps(num_nodes):
    (N, M, P) = num_nodes
    for i in [0, 2 * N]:
        clamped_i = i + 0.5 if i == 0 else i - 0.5
        for j in [0, 2 * M]:
            clamped_j = j + 0.5 if j == 0 else j - 0.5
            for k in range(1, 2 * P):
                if k % 2 == 1:
                    yield ((clamped_i, clamped_j, k - 0.5), (clamped_i, clamped_j, k + 0.5))

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
    num_nodes = (10, 20, 35)

    writer = SegmentWriter()
    writer.add_segments(all_cross_tiles(num_nodes))
    writer.add_segments(all_2d_caps(num_nodes))
    writer.add_segments(all_edge_caps(num_nodes))
    writer.write('celtic_tiles.obj')
