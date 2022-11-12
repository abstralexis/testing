# This is a test program I got from Stack Overflow, tweaked, and added
# comments for rendering LaTeX to the screen. I plan to make a discord
# bot eventually that returns a rendered image of maths stuff
# such that a user can pass a LaTeX syntax string as an argument 
# and get a better representation of maths in a conversation.
#
# Make sure you have TeX Live installed first... and that takes a while
# Or install MiKTeX like a sane person (much better)

import matplotlib.pyplot as plt # Main plotting stuff  
from matplotlib import rcParams # Thingy for changing parameters n stuff
rcParams['text.usetex'] = True  # Set it so that you can use LaTeX rendering

plt.style.use('dark_background')    # Dark mode !!!

latex_expression = r"""
To translate coordinates from Computer Graphics to
Cartesian, with the origin at the centre, given the
screen width ($w$) and height ($h$):
$f(a, b) = (a - \frac{w}{2}, b - \frac{h}{2}), 0 \leq a \leq w, 0 \leq b \leq h$
Therefore:
$f'(a, b) = (a + \frac{w}{2}, b + \frac{h}{2}), -\frac{w}{2} \leq a \leq \frac{w}{2}, -\frac{h}{2} \leq b \leq \frac{h}{2}$
""" # Some LaTeX expr that you can pass

plt.text(0.01, 0.5, latex_expression, fontsize=18)   # Render the LaTeX text

# Axes stuff
ax = plt.gca()  # Get the state of the current axes
# Set the axes to not be visible
ax.axes.get_xaxis().set_visible(False)
ax.axes.get_yaxis().set_visible(False)

# Show the plot
plt.draw()
plt.show()

# I think that in the final program, I will have to do some fancy maths
# to figure out what the font size should be and also text wrapping and such.
# Maybe also editing the image from matplotlib with pillow to look nicer.
