import qualified Data.Set as Set

type Point = (Int,Int)

data Line = Line Point Point
    deriving (Show, Eq)

data Move = Up Int | Left' Int | Down Int | Right' Int
    deriving (Show, Eq)

intersection :: (Ord a) => [a] -> [a] -> [a]
intersection xs ys =
    let set1 = Set.fromList xs
        set2 = Set.fromList ys
    in Set.toList $ Set.intersection set1 set2

range :: (Enum a,Ord a) => a -> a -> [a]
range l u
    | l < u = [l .. u]
    | l > u = [u .. l]
    | otherwise = [l]

getIntersections :: Line -> Line -> [Point]
getIntersections line1 line2 =
    let xints = intersection (xlist line1) (xlist line2)
        yints = intersection (ylist line1) (ylist line2)
    in case (xints,yints) of
        ([x],[y]) -> [(x,y)]
        ([x],ys) -> zip (replicate (length ys) x) ys
        (xs,[y]) -> zip xs (replicate (length xs) y)
        _ -> []
    where xlist (Line (x1,_) (x2,_)) = range x1 x2
          ylist (Line (_,y1) (_,y2)) = range y1 y2

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

solidify :: [Point] -> [Line]
solidify (p1:p2:rest_points) = (Line p1 p2) : (solidify (p2:rest_points))
solidify _ = []

lineIntersectionPath :: Line -> [Line] -> [Point]
lineIntersectionPath line path = case path of
    [] -> []
    (fp:rest_path) -> (getIntersections line fp) ++ lineIntersectionPath line rest_path

pathIntersectionPath :: [Line] -> [Line] -> [Point]
pathIntersectionPath path1 path2 = foldl (\acc x -> acc ++ lineIntersectionPath x path2) [] path1

-- Calculate the Manhattan distance of a point wrt origin
computeDistance :: Point -> Int
computeDistance (x,y) = x + y

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

main = do
    inputs <- sequence $ replicate 2 getLine
    let paths = (map solidify . map (getPath centralPort) . map parseInput) inputs
        path1 = head paths
        path2 = last paths
    print $ minimum $ tail $ map computeDistance $ pathIntersectionPath path1 path2
    where centralPort = (0,0)
