def exchange_money(budget, exchange_rate):
    """

    :param budget: float - amount of money you are planning to exchange.
    :param exchange_rate: float - unit value of the foreign currency.
    :return: float - exchanged value of the foreign currency you can receive.
    """
    return budget / exchange_rate
    


def get_change(budget, exchanging_value):
    """

    :param budget: float - amount of money you own.
    :param exchanging_value: float - amount of your money you want to exchange now.
    :return: float - amount left of your starting currency after exchanging.
    """

    return budget - exchanging_value


def get_value_of_bills(denomination, number_of_bills):
    """

    :param denomination: int - the value of a bill.
    :param number_of_bills: int - amount of bills you received.
    :return: int - total value of bills you now have.
    """

    return denomination * number_of_bills


def get_number_of_bills(budget, denomination):
    """

    :param budget: float - the amount of money you are planning to exchange.
    :param denomination: int - the value of a single bill.
    :return: int - number of bills after exchanging all your money.
    """

    return budget // denomination


def get_leftover_of_bills(budget, denomination):
    """

    : Param Budget: Float - сумма денег, которую вы планируете обменять. 
     : Парам деноминация: int - значение одного счета. 
     : return: float - оставшаяся сумма, которая не может быть обменена, учитывая текущую деноминацию.
    """
    return get_change(budget, get_value_of_bills(denomination, get_number_of_bills(budget, denomination)))


def exchangeable_value(budget, exchange_rate, spread, denomination):
    """

    :ПАРАМ Бюджет: Float - сумма ваших денег, которые вы планируете обменять.
: param Exchange_Rate: Float - значение единицы иностранной валюты. 
     : param press: int - процент, который принимается в качестве обменного плата. 
     : Парам деноминация: int - значение одного счета. 
     : return: int - максимальное значение, которое вы можете получить.
    """
    new_budget = exchange_money(budget, exchange_rate * (100 + spread) / 100)
    return get_value_of_bills(denomination, get_number_of_bills(new_budget, denomination))
