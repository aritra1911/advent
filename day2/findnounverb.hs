import Intcode

tryNounVerb :: (Int,Int) -> Memory -> Memory
tryNounVerb (noun,verb) (x:_:_:rest) = x:noun:verb:rest
tryNounVerb _ program = program

getNounVerb :: Memory -> Int -> [(Int,Int)] -> Maybe (Int,Int)
getNounVerb _ _ [] = Nothing
getNounVerb program givenOutput (nv:nvls) =
    let output = (getOutput . run . initProgram . (tryNounVerb nv)) program
    in if output /= givenOutput
        then getNounVerb program givenOutput nvls
        else Just nv

find :: Memory -> Int -> Either String Int
find program output =
    let range  = [0 .. ((length program) - 1)]
        nvlist = [(x,y) | x <- range, y <- range]
    in case getNounVerb program output nvlist of
        Nothing -> Left "Not found!"
        Just (noun,verb) -> Right $ 100 * noun + verb

main = do
    program <- readFile "input.txt"
    output  <- getLine
    case find (parse program) (read output) of
        Left errorMsg -> putStrLn errorMsg
        Right nv -> putStrLn $ show nv
