from matplotlib import pyplot as plt
import numpy as np


# Parameters
vmax = 32 
vs = 0.01
tmax = 0.1
ts = 0.2 

# Computed parameters
f = np.log(vmax / vs) / ts
print("f = " + str(f))
a = np.log(vmax / vs) / tmax
print("a = " + str(a))

def simulate(dt, initial, expr, stop_cond):
    sim_t = 0
    t = [0]
    v = [initial]
    while not stop_cond(sim_t, v[-1]):
        sim_t += dt
        t.append(sim_t)
        v.append(expr(dt, v[-1]))
    return (sim_t, t, v)

def graph(start, end, nb_pts, expr):
    step = (end - start)/ nb_pts
    xs = [start]
    ys = [expr(start)]
    while xs[-1] < end:
        xs.append(xs[-1] + step)
        ys.append(expr(xs[-1]))
    return (xs, ys)

def decay(a, b, l, dt):
    return b + (a - b) * np.exp(-l * dt)


# Deceleration graph
(r_ts, td, vd) = simulate(0.01, vmax, lambda dt, vp: decay(vp, 0, f, dt), lambda sim_t, v: v < vs )
(td_th, vd_th) = graph(0, ts, 100, lambda t: vmax * np.exp(-f * t))
print("simulated ts: " + str(r_ts))
print("theoritical ts: " + str(ts))

fig, axs = plt.subplots(2)
axs[0].plot(td, vd)
axs[0].set_title("simulated deceleration")
axs[1].plot(td_th, vd_th)
axs[1].set_title("theoritical deceleration")

plt.show()


