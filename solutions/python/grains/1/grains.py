def square(number):
    if 1 <= number and number <= 64:
        return 2 ** (number - 1)
    else:
        raise ValueError("square must be between 1 and 64")


def total():
    square_number = 1
    total_grains = 0
    while square_number <= 64:
        total_grains += square(square_number)
        square_number += 1
    return total_grains 
