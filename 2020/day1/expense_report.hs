findEntries :: Int -> [Int] -> Maybe (Int, Int)
findEntries _ [] = Nothing
findEntries _ (_:[]) = Nothing
findEntries year (x:xs) =
    case filter (\a -> (x + a) == year) xs of
        [] -> findEntries year xs
        (y:ys) -> Just (x,y)

findThreeEntries :: Int -> [Int] -> Maybe (Int, Int, Int)
findThreeEntries _ [] = Nothing
findThreeEntries _ (_:[]) = Nothing
findThreeEntries year (x:xs) =
    case findEntries (year - x) xs of
        Nothing -> findThreeEntries year xs
        Just (y,z) -> Just (x,y,z)

multiplyEntries :: Maybe (Int, Int, Int) -> Either String Int
multiplyEntries Nothing = Left "None"
multiplyEntries (Just (x,y,z)) = Right $ x * y * z

showOutput :: Either String Int -> String
showOutput value = case value of Left error -> error
                                 Right product -> show product

main = do
    input <- readFile "input.txt"
    putStrLn $ (showOutput
              . multiplyEntries
              . findThreeEntries year
              . map read 
              . lines) input
    where year = 2020
