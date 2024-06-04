# Spokers a fast paced arena FPS.

player physics:
- Get player input.
- Calculate player wish direction.
- Calculate player wish direction.

## full player update loop.

1) compute player state:
find if player colliding with a hardsurface. (Ground, Walls, Ceiling) set relevant flags.
find wich empty area the player is in and set relevant flags.

2) Get player input and compute wish direction.

3) Accelerate the player.

4) Move the player.

5) Run the Colision detection and resove.


calculate_accel()
p'(t) = p''(t-1) * dt * .5
resolve_colisions()
p(t) = p'(t)
p'(t) = p''(t-1) * dt * .5

## Verlet
p[t] += v[t-1] * dt + 1/2 * a[t-1] * dt * dt
a[t] = compute new acc
v[t] += 1/2 (a[t-1] + a[t]) * dt
## Real time csg.

Algorithm.

Requisites:
- A brush is represented by the half edge data structure.
- All polygons of a brush must be convex.

1) Find intersecting brushes.
    - Explore Heirarchical Hash Grid.
2) Process polygons two by two.
    - Find polygons that are formed at the boundary between the two brushes. For each brush:
        - Find all vertices that are inside or on the boundary of the other brush.
        - For all vertices that are inside the other brush find the edge that intersect the boundary of the other brush (keep a reference to the edge somewhere).
        - Create intesection polygons:
            - If all the vertices of a polygon lie inside the boundary of the other brush the entire polygon is intersecting.
            - Otherwise reconstruct the intersection polygon by finding the twin edges (half edge in the neighbor polygon) intersecting the other brush polygon. 
