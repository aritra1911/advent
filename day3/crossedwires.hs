parseInput :: String -> [String]
parseInput = words . (map commasToSpaces)
    where commasToSpaces c = if c == ',' then ' ' else c

main = interact $ unlines . (map show) . (map parseInput) . lines
