type Program = [Int]

-- pointer points to the instruction to be read
data ProgramState = ProgramState { pointer :: Int
                                 , program :: Program
                                 } deriving (Show, Eq)

data OpCode = NOP | ADD | MUL | HLT deriving (Show,Eq,Ord)

-- ProgramStates don't care about the program itself
instance Ord ProgramState where
    (ProgramState p _) `compare` (ProgramState q _) = p `compare` q

-- https://hackage.haskell.org/package/ilist-0.4.0.1/docs/Data-List-Index.html#v:setAt
setAt :: Int -> a -> [a] -> [a]
setAt i a ls
  | i < 0 = ls
  | otherwise = go i ls
  where
    go 0 (_:xs) = a : xs
    go n (x:xs) = x : go (n-1) xs
    go _ []     = []

intToOpcode :: Int -> OpCode
intToOpcode instruction = case instruction of
    1  -> ADD
    2  -> MUL
    99 -> HLT
    _  -> NOP

-- TODO: HLT /= NOP
eval :: OpCode -> Int -> Int -> Maybe Int
eval ADD x y = Just $ x + y
eval MUL x y = Just $ x * y
eval HLT _ _ = Nothing
eval NOP _ _ = Nothing

execute :: ProgramState -> ProgramState
execute (ProgramState ptr prog) =
    let code = intToOpcode $ prog !! ptr
        result = eval code (val $ arg 1) (val $ arg 2)
    in case result of
        Just x -> execute $ ProgramState {pointer = ptr + 4, program = setAt (arg 3) x prog}
        Nothing -> ProgramState {pointer = 0, program = prog}
    where arg x = prog !! (ptr + x)
          val x = prog !! x

getOutput :: ProgramState -> String
getOutput (ProgramState _ state) = show $ state !! 0

initExec :: Program -> ProgramState
initExec prog = ProgramState {pointer = 0, program = prog}

parse :: String -> Program
parse input = read $ '[':input ++ "]"

restore1202 :: Program -> Program
restore1202 (x:_:_:rest) = x:12:02:rest

main = do
    input <- getContents
    putStrLn $ (getOutput . execute . initExec . restore1202 . parse) input
