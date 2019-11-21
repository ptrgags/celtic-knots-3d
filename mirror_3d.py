#!/usr/bin/env python3

def get_bounds(num_nodes):
    (N, M, P) = num_nodes
    x_bounds = (0.0, 2.0 * N)
    y_bounds = (0.0, 2.0 * M)
    z_bounds = (0.0, 2.0 * P)
    return (x_bounds, y_bounds, z_bounds)

def shrink_bounds(bounds, amount):
    return tuple(
        (min_val + amount, max_val - amount)
        for (min_val, max_val) in bounds)

def in_bounds(point, bounds):
    for x, (min_val, max_val) in zip(point, bounds):
        if x < min_val or max_val < x:
            return False
    else:
        return True

def clamp(x, a, b):
    return min(max(x, a), b)

def clamp_bounds(point, bounds):
    return tuple(
        clamp(x, min_val, max_val) 
        for x, (min_val, max_val) in zip(point, bounds))

def follow_ray(point, direction, step_size):
    return tuple(
        x + step_size * v
        for x, v in zip(point, direction))

def bounce_at_boundary(point, direction, bounds):
    return tuple(
        -v if x == min_val or x == max_val else v
        for x, v, (min_val, max_val) in zip(point, direction, bounds))

def reflect(num_nodes, start_point, start_direction):
    STEP_SIZE = 0.5
    bounds = get_bounds(num_nodes)
    smaller_bounds = shrink_bounds(bounds, STEP_SIZE)
    current_point = start_point
    current_direction = start_direction
    while True:
        yield clamp_bounds(current_point, smaller_bounds)
        current_point = follow_ray(current_point, current_direction, STEP_SIZE)
        current_direction = bounce_at_boundary(
            current_point, current_direction, bounds)
        if current_point == start_point and current_direction == start_direction:
            break;

class PolylineWriter:
    def __init__(self):
        self.vertices = []
        self.polylines = []

    def add_polyline(self, vertex_seq):
        start_index = len(self.vertices)

        vertices = list(vertex_seq)
        indices = [(start_index + i) + 1 for i, _ in enumerate(vertices)]
        indices.append(indices[0]) # close the curve

        self.vertices.extend(vertices)
        self.polylines.append(indices)

    def write(self, fname):
        with open(fname, 'w') as f:
            for x, y, z in self.vertices:
                f.write(f'v {x} {y} {z}\n')

            for polyline in self.polylines:
                indices_str = ' '.join(str(i) for i in polyline)
                f.write(f'l {indices_str}\n')

if __name__ == '__main__':
    num_nodes = (4, 2, 3)
    start_point = (3.0, 1.0, 2.0)
    start_direction = (1.0, 1.0, 1.0)

    writer = PolylineWriter()
    writer.add_polyline(reflect(num_nodes, start_point, (1.0, 1.0, 1.0)))
    writer.add_polyline(reflect(num_nodes, start_point, (1.0, -1.0, 1.0)))
    writer.add_polyline(reflect(num_nodes, (3.0, 3.0, 2.0), (1.0, 1.0, 1.0)))
    writer.add_polyline(reflect(num_nodes, (3.0, 3.0, 2.0), (1.0, -1.0, 1.0)))
    writer.write('mirror_curve.obj')

