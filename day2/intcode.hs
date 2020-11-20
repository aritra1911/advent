module Intcode
( Memory
, State(..)
, run
, initProgram
, getOutput
, parse
) where

type Memory = [Int]

-- Here `pointer` refers to the instruction pointer
data State = State { pointer :: Int
                   , memory  :: Memory
                   } deriving (Show, Eq)

data OpCode = NOP | ADD | MUL | HLT deriving (Show, Eq, Ord, Bounded, Enum)

-- Memory contents doesn't matter when comparing states
instance Ord State where
    state1 `compare` state2 = (pointer state1) `compare` (pointer state2)

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

stepVal :: OpCode -> Int
stepVal NOP = 1
stepVal HLT = 1
stepVal _   = 4

-- TODO: HLT /= NOP
eval :: OpCode -> Int -> Int -> Maybe Int
eval ADD x y = Just $ x + y
eval MUL x y = Just $ x * y
eval HLT _ _ = Nothing
eval NOP _ _ = Nothing

run :: State -> State
run state =
    let opcode  = intToOpcode $ (memory state) !! (pointer state)
        nextPtr = (pointer state) + (stepVal opcode)
        result  = eval opcode (val $ arg 1) (val $ arg 2)
    in case result of
        Just x  -> run $ State { pointer = nextPtr
                               , memory  = setAt (arg 3) x (memory state)
                               }
        Nothing -> State { pointer = nextPtr
                         , memory = memory state
                         }
    where arg x = (memory state) !! ((pointer state) + x)
          val x = (memory state) !! x

initProgram :: Memory -> State
initProgram contents = State { pointer = 0, memory = contents }

getOutput :: State -> Int
getOutput state = (head . memory) state

parse :: String -> Memory
parse input = read $ '[':input ++ "]"
