from ..mod import romanize


def test_romanize():
    assert romanize(1) == 'one'
    assert romanize(2) == 'two'
    assert romanize(3) != 'two'

