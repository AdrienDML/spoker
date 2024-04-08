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


# Aery patterns

