calculateFuel :: Int -> Int
calculateFuel mass =
    let fuel = ((mass `div` 3) - 2 )
    in if fuel <= 0
        then 0
        else fuel + calculateFuel fuel

fuelRequirements :: [Int] -> Int
fuelRequirements = sum . map calculateFuel

main = do
    contents <- getContents
    print $ (fuelRequirements . map read . lines) contents
