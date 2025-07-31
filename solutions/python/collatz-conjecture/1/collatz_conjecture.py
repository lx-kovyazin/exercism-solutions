def steps(number):
    match number:
        case _ if number <= 0:
            raise ValueError("Only positive integers are allowed")
        case 1:
            return 0
        case even_number if number % 2 == 0:
            next_number = even_number / 2
        case odd_number:
            next_number = odd_number * 3 + 1
    return steps(next_number) + 1
