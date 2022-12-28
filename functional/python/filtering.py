"""
This illustrates the concept of a filter.
"""

# Make a list from a range object
some_list = list(range(0, 100))

# Filter with a lambda function that returns a bool
# The item gets filtered out if the anonymous function 
# returns False.
evens = list(filter(lambda x: x % 2 == 0, some_list))

# Print
print(some_list)
print(evens)