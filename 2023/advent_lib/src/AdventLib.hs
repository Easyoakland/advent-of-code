module AdventLib (checkResult) where

-- | Checks expected vs actual value. First argument is description of testcase
checkResult :: (Eq a, Show a) => String -> a -> IO a -> IO ()
checkResult testCase expected res_f = do
  res <- res_f
  if res /= expected
    then error $ testCase ++ " fail. Result: " ++ show res ++ " != Expected: " ++ show expected
    else putStrLn $ testCase ++ " pass: " ++ show res
