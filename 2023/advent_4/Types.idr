module Types

namespace Card
    public export
    record Card where
        constructor Mk
        winning_nums : List Nat
        have_nums: List Nat

    export
    Interpolation Card where
        interpolate c = interpolate """
            Card {
                winning_nums = \{show c.winning_nums}
                have_nums = \{show c.have_nums}
            }
        """
    export
    Show Card where show = interpolate

