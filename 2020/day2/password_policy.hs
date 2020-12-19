data PasswordRecord = PasswordRecord {  atLeast :: Int
                                     ,   atMost :: Int
                                     , passChar :: Char
                                     , password :: String
                                     } deriving (Show)

getAtLeast :: String -> Int
getAtLeast = read . takeWhile (\x -> x /= '-')

getAtMost :: String -> Int
getAtMost pwdrec = case (tail . head . words) pwdrec of
                 ('-':atmost) -> read atmost
                 _ -> (getAtMost . tail) pwdrec

getPassChar :: String -> Char
getPassChar pwdrec = head $ (words pwdrec) !! 1

getPassword :: String -> String
getPassword = drop 2 . dropWhile (\x -> x /= ':')

isValidRecord :: PasswordRecord -> Bool
isValidRecord passrec = (occurences >= atLeast passrec)
                     && (occurences <= atMost passrec)
    where occurences = length
                     $ filter (\x -> x == passChar passrec) (password passrec)

parseRecord :: String -> PasswordRecord
parseRecord pwd = PasswordRecord {  atLeast = getAtLeast pwd
                                 ,   atMost = getAtMost pwd
                                 , passChar = getPassChar pwd
                                 , password = getPassword pwd
                                 }

main = do
    input <- readFile "input.txt"
    let passrecs = (map parseRecord . lines) input
    print $ (length . filter isValidRecord) passrecs
