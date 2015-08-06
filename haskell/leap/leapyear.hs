module LeapYear (isLeapYear, isLeapYear2) where

isLeapYear :: Int -> Bool
isLeapYear year
    | year `mod` 100 == 0 && year `mod` 400 /= 0  = False
    | year `mod` 4 == 0  =  True
    | otherwise = False

isLeapYear2 :: Int -> Bool
isLeapYear2 year
    | year `mod` 4 /= 0  = False
    | year `mod` 100 == 0 && year `mod` 400 /= 0 = False
    | otherwise = True
