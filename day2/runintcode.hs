import Intcode

restore :: Int -> Memory -> Memory
restore nounverb (x:_:_:rest) =
    let noun = nounverb `div` 100
        verb = nounverb `mod` 100
    in x:noun:verb:rest
restore _ program = program

main = do
    input <- readFile "input.txt"
    print $ (getOutput . run . initProgram . (restore 1202) . parse) input
