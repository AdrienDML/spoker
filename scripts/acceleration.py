import matplotlib.pyplot as plt
import numpy as np

from matplotlib.widgets import Button, Slider
import math

max_speed = 32.0
dt = 0.1
tmax = 0.5
acceleration =  max_speed / tmax

def delta(angle, speed, added_speed):
    return speed - np.sqrt(speed**2 + added_speed**2 - 2 * math.cos(angle) * speed * added_speed)

def strafe(angle, speed):
    projected_speed = math.cos(angle) * speed
    added_speed = np.clip(max_speed - projected_speed, 0.0, acceleration * dt)
    return delta(angle, speed, added_speed)

def bunny_hop(angle, speed):
    projected_speed = math.cos(angle) * speed
    added_speed = acceleration * dt
    if projected_speed + added_speed > max_speed:
        added_speed = max_speed - projected_speed
    return delta(angle, speed, added_speed)

def map(angle, f, speed):
    new_speed = np.ndarray(angle.shape)
    for idx in range(len(angle)):
        new_speed[idx] = f(angle[idx], speed)
    return new_speed

# The parametrized function to be plotted
def f(angle, speed):
    sa = map(angle, strafe, speed)
    ba = map(angle, bunny_hop, speed)
    return np.maximum(sa, ba)

angle = np.linspace(-np.pi, np.pi, 1000)
init_speed = 0

# Create the figure and the line that we will manipulate
fig, ax = plt.subplots()
#proj_speed, = ax.plot(angle, np.cos(angle) * init_speed, c='y')
#max, = ax.plot(angle, f(angle, init_speed), c='r', label='max')
s, = ax.plot(angle, map(angle, strafe, init_speed), c='g', label='strafe')
b, = ax.plot(angle, map(angle, bunny_hop, init_speed), c='b', label='bunny_hop')
ax.autoscale(True)
ax.set_xlabel('Angle')
ax.set_ylabel('Speed')
plt.legend()

# adjust the main plot to make room for the sliders
fig.subplots_adjust(bottom=0.25)

# Make a horizontal slider to control the frequency.
axfreq = fig.add_axes([0.25, 0.1, 0.65, 0.03])
speed_slider = Slider(
    ax=axfreq,
    label='Current Speed',
    valmin=0,
    valmax=2*max_speed,
    valinit=0,
)


# The function to be called anytime a slider's value changes
def update(val):
    #proj_speed.set_ydata(np.cos(angle) * speed_slider.val)
    #max.set_ydata(f(angle, speed_slider.val))
    s.set_ydata(map(angle, strafe, speed_slider.val))
    b.set_ydata(map(angle, bunny_hop, speed_slider.val))
    ax.relim()
    ax.autoscale()
    fig.canvas.draw_idle()


# register the update function with each slider
speed_slider.on_changed(update)

plt.show()
# Create a `matplotlib.widgets.Button` to reset the sliders to initial values.
