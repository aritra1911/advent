import Intcode

getOutput :: State -> String
getOutput state = show $ (memory state) !! 0

initProgram :: Memory -> State
initProgram contents = State { pointer = 0, memory = contents }

parse :: String -> Memory
parse input = read $ '[':input ++ "]"

restore1202 :: Memory -> Memory
restore1202 (x:_:_:rest) = x:12:02:rest
restore1202 mem = mem

main = do
    input <- getContents
    putStrLn $ (getOutput . run . initProgram . restore1202 . parse) input
