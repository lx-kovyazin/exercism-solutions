def eat_ghost(power_pellet_active, touching_ghost):
    """_summary_

    Args:
        power_pellet_active (_type_): _description_
        touching_ghost (_type_): _description_

    Returns:
        _type_: _description_
    """
    return power_pellet_active and touching_ghost


def score(touching_power_pellet, touching_dot):
    """_summary_

    Args:
        touching_power_pellet (_type_): _description_
        touching_dot (_type_): _description_

    Returns:
        _type_: _description_
    """
    return touching_power_pellet or touching_dot


def lose(power_pellet_active, touching_ghost):
    """_summary_

    Args:
        power_pellet_active (_type_): _description_
        touching_ghost (_type_): _description_

    Returns:
        _type_: _description_
    """
    return not power_pellet_active and touching_ghost


def win(has_eaten_all_dots, power_pellet_active, touching_ghost):
    """_summary_

    Args:
        has_eaten_all_dots (bool): _description_
        power_pellet_active (_type_): _description_
        touching_ghost (_type_): _description_

    Returns:
        _type_: _description_
    """
    return (power_pellet_active or not touching_ghost) and has_eaten_all_dots
