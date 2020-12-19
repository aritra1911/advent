data PasswordRecord = PasswordRecord {  firstIndex :: Int
                                     , secondIndex :: Int
                                     ,    passChar :: Char
                                     ,    password :: String
                                     } deriving (Show)

getFirstIndex :: String -> Int
getFirstIndex = read . takeWhile (\x -> x /= '-')

getSecondIndex :: String -> Int
getSecondIndex pwdrec = case (tail . head . words) pwdrec of
                 ('-':index) -> read index
                 _ -> (getSecondIndex . tail) pwdrec

getPassChar :: String -> Char
getPassChar pwdrec = head $ (words pwdrec) !! 1

getPassword :: String -> String
getPassword = drop 2 . dropWhile (\x -> x /= ':')

isValidRecord :: PasswordRecord -> Bool
isValidRecord passrec =
    let firstChar = (password passrec) !! ((firstIndex passrec) - 1)
        secondChar = (password passrec) !! ((secondIndex passrec) - 1)
        char = passChar passrec
    in (firstChar /= secondChar) && (char == firstChar || char == secondChar)

parseRecord :: String -> PasswordRecord
parseRecord pwd = PasswordRecord {  firstIndex = getFirstIndex pwd
                                 , secondIndex = getSecondIndex pwd
                                 ,    passChar = getPassChar pwd
                                 ,    password = getPassword pwd
                                 }

main = do
    input <- readFile "input.txt"
    let passrecs = (map parseRecord . lines) input
    print $ (length . filter isValidRecord) passrecs
