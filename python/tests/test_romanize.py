from ..mod import romanize


def test_romanize():
    assert romanize(1) == 'one'
    assert romanize(3) != 'two'

