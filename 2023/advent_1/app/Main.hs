module Main where

import Data.List (isPrefixOf, isSuffixOf, uncons)
import Data.Maybe (fromJust)
import GHC.Unicode (isDigit)

part1 :: FilePath -> IO Int
part1 path = do
  file <- readFile path
  let numbers = map (filter isDigit) $ lines file -- extract numbers
  let results = map (\x -> [head x, last x]) numbers -- first,last only
  return $ sum $ map read results

-- | Find if text version of a digit exists in the given string using given searcher
wordNumber :: (String -> String -> Bool) -> String -> Maybe Char
wordNumber searcher str
  | searcher "one" str = Just '1'
  | searcher "two" str = Just '2'
  | searcher "three" str = Just '3'
  | searcher "four" str = Just '4'
  | searcher "five" str = Just '5'
  | searcher "six" str = Just '6'
  | searcher "seven" str = Just '7'
  | searcher "eight" str = Just '8'
  | searcher "nine" str = Just '9'
  | otherwise = Nothing

-- | Find first number or digit in word form in string
fstNumber :: String -> Maybe Char
fstNumber [] = Nothing
fstNumber str = do
  case wordNumber isPrefixOf str of
    Just res -> return res
    Nothing -> do
      (c, rest) <- uncons str
      if isDigit c
        then return c
        else fstNumber rest

-- | Find last number or digit in word form in string
lstNumber :: String -> Maybe Char
lstNumber [] = Nothing
lstNumber str = do
  case wordNumber isSuffixOf str of
    Just res -> return res
    Nothing -> do
      (c, rest) <- uncons . reverse $ str
      if isDigit c
        then return c
        else lstNumber $ reverse rest

part2 :: FilePath -> IO Int
part2 path = do
  file <- readFile path
  let firsts = map (fromJust . fstNumber) $ lines file
  let lasts = map (fromJust . lstNumber) $ lines file
  let numbers = zipWith (\x y -> [x, y]) firsts lasts
  return $ sum $ map read numbers

-- | Checks function outputs correct value. First argument is description of testcase
checkResult :: (Eq a, Show a) => String -> a -> IO a -> IO ()
checkResult testCase expected res1 = do
  -- res1 >>= \res -> -- same line below
  res <- res1
  if res /= expected
    then error $ testCase ++ " fail. Result: " ++ show res ++ " != Expected: " ++ show expected
    else putStrLn $ testCase ++ " pass: " ++ show res

main :: IO ()
main = do
  --   p1_test <- part1 "inputtest.txt"
  --   when (p1_test /= 142) $ error $ "part1 test result: " ++ show p1_test
  -- OR
  --   part1 "inputtest.txt" >>= \res -> when (res /= 142) $ error $ "part1 test result: " ++ show res
  checkResult "part1 test" 142 $ part1 "inputtest.txt"
  checkResult "part1" 55447 $ part1 "input.txt"
  checkResult "part2 test" 281 $ part2 "inputtest2.txt"
  checkResult "part2" 54706 $ part2 "input.txt"
