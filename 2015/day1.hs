neededFloor :: [Char] -> Int
neededFloor parentheses = 
    sum [ 
        if p == '(' then 1 
        else -1 | p <- parentheses 
    ]

firstToBasement :: [Char] -> Int
firstToBasement parentheses = 
    head [
        snd pair |
        pair <- [
            (take len parentheses, len) |
            len <- [1..length parentheses]
        ],
        neededFloor (fst pair) < 0
    ]
