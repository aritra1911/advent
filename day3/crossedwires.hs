type Point = (Int,Int)

data Move = Up Int | Left' Int | Down Int | Right' Int
    deriving (Show, Eq)

nudge :: Point -> Move -> Point
nudge (x,y) move =
    case move of
        Up     dist -> (x,y + dist)
        Left'  dist -> (x - dist,y)
        Down   dist -> (x,y - dist)
        Right' dist -> (x + dist,y)

getPath :: Point -> [Move] -> [Point]
getPath start [] = [start]
getPath start (move:rest_moves) = start : (getPath newPoint rest_moves)
    where newPoint = start `nudge` move

-- These two functions do the same exact thing. I personally prefer the above recursive implementation because of it's
-- verbosity and it keeps it in order, but I'm keeping the below one as well just for the sake of demonstration.

getPath' :: Point -> [Move] -> [Point]
getPath' start moves = (reverse . foldl stitch [start]) moves
    where stitch (point:rest_points) move = (point `nudge` move) : point : rest_points

parseMove :: String -> Move
parseMove (move:distance) =
    case move of
        'U' -> Up     $ read distance
        'L' -> Left'  $ read distance
        'D' -> Down   $ read distance
        'R' -> Right' $ read distance

parseInput :: String -> [Move]
parseInput = map parseMove . words . map commaToSpace
    where commaToSpace c = if c == ',' then ' ' else c

main = interact $ unlines . map show . map (getPath centralPort) . map parseInput . lines
    where centralPort = (0,0)
