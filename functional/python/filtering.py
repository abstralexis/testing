"""
This illustrates the concept of a filter.
"""

some_list = list(range(0, 100))
evens = list(filter(lambda x: x % 2 == 0, some_list))
print(some_list)
print(evens)