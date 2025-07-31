EXPECTED_BAKE_TIME = 40
PREPARATION_TIME = 5
"""A constant equal to the time it takes to prepare a single layer."""

TIME_PER_LAYER = 2


def bake_time_remaining(elapsed_bake_time):
    """Calculate the bake time remaining.

    :param elapsed_bake_time: int - baking time already elapsed.
    :return: int - remaining bake time (in minutes) derived from 'EXPECTED_BAKE_TIME'.

    Function that takes the actual minutes the lasagna has been in the oven as
    an argument and returns how many minutes the lasagna still needs to bake
    based on the `EXPECTED_BAKE_TIME`.
    """
    return EXPECTED_BAKE_TIME - elapsed_bake_time


def preparation_time_in_minutes(layer):
    """"""
    return layer * TIME_PER_LAYER


def elapsed_time_in_minutes(layer, bake_time_remaining):
    """"""
    PREPARATION_TIME = preparation_time_in_minutes(layer)
    return PREPARATION_TIME + bake_time_remaining
