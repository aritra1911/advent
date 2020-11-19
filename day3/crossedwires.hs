data Move d = Up d | Left' d | Down d | Right' d
    deriving (Show, Eq)

parseMove :: String -> Move Int
parseMove (move:distance) =
    case move of
        'U' -> Up     $ read distance
        'L' -> Left'  $ read distance
        'D' -> Down   $ read distance
        'R' -> Right' $ read distance

parseInput :: String -> [Move Int]
parseInput = map parseMove . words . map commasToSpaces
    where commasToSpaces c = if c == ',' then ' ' else c

main = interact $ unlines . map show . map parseInput . lines
