type Program = [Int]

-- pointer points to the instruction to be read
data ProgramState = ProgramState { pointer :: Int
                                 , program :: Program
                                 } deriving (Show, Eq)

data OpCode = NOP | ADD | MUL | HLT deriving (Show,Eq,Ord)

-- ProgramStates don't care about the program itself
instance Ord ProgramState where
    (ProgramState p _) `compare` (ProgramState q _) = p `compare` q

intToOpcode :: Int -> OpCode
intToOpcode instruction = case instruction of
    1  -> ADD
    2  -> MUL
    99 -> HLT
    _  -> NOP

restore1202 :: Program -> Program
restore1202 (x:_:_:rest) = x:12:02:rest
