""" helper functions """

def title_to_slug(title: str) -> str:
    """
    :param title: (str) e.g. 'A Very Good Book'
    :return: (str) 'a-very-good-book'
    """
    return '-'.join(title.lower().split())