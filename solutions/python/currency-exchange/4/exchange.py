def exchange_money(budget, exchange_rate):
    """1"""
    return budget / exchange_rate  


def get_change(budget, exchanging_value):
    """1"""
    return budget - exchanging_value


def get_value_of_bills(denomination, number_of_bills):
    """1"""
    return denomination * number_of_bills


def get_number_of_bills(budget, denomination):
    """1"""
    return budget // denomination


def get_leftover_of_bills(budget, denomination):
    """1"""
    return get_change(budget, get_value_of_bills(denomination, get_number_of_bills(budget, denomination)))


def exchangeable_value(budget, exchange_rate, spread, denomination):
    """1"""
    new_budget = exchange_money(budget, exchange_rate * (100 + spread) / 100)
    return get_value_of_bills(denomination, get_number_of_bills(new_budget, denomination))