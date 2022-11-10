def romanize(v: int) -> str:
    match v:
        case 1:
            return 'one'
        case 2:
            return 'two'
        case 3:
            return 'three'
        case 4:
            return 'four'
        case 5:
            return 'five'
        case 6:
            return 'six'
        case 7:
            return 'seven'
        case _:
            raise ValueError('only 1 to 7 is allowed')


