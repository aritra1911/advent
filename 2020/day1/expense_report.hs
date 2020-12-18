findEntries :: Int -> [Int] -> Maybe (Int, Int)
findEntries _ [] = Nothing
findEntries _ (_:[]) = Nothing
findEntries year (x:xs) =
    case filter (\a -> (x + a) == year) xs of
        [] -> findEntries year xs
        (y:ys) -> Just (x,y)

multiplyEntries :: Maybe (Int, Int) -> Either String Int
multiplyEntries Nothing = Left "None"
multiplyEntries (Just (e1, e2)) = Right $ e1 * e2

showOutput :: Either String Int -> String
showOutput value = case value of Left error -> error
                                 Right product -> show product

main = do
    input <- readFile "input.txt"
    putStrLn $ (showOutput
              . multiplyEntries
              . findEntries year 
              . map read 
              . lines) input
    where year = 2020
