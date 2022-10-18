# Some basic matplotlib stuff. There are many similarities
# with MATLAB, so this is either really painful or the best
# thing in the world depending on who you ask.

import numpy as np
from matplotlib import pyplot as plt

plt.style.use("dark_background")

# Set the x variable for the graph
x = np.arange(-4*np.pi,4*np.pi,0.1)

f = np.sin(x)
g = (1/100)*x**3 + 2*x + 4

# Plot the axes manually because it has to be this way for some reason
plt.axvline(x=0, color="gray")  # Vertical axis
plt.axhline(y=0, color="gray")  # Horizontal axis

# Plot in the linear space x the function f(x) in red.
plt.plot(x, f, color="red")
plt.plot(x, g, color="blue")

for h, k in set(f) & set(g):
    plt.plot(h, k, "marker")

plt.show()