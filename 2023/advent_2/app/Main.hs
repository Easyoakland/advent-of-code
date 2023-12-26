{-# LANGUAGE NamedFieldPuns #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE ScopedTypeVariables #-}

module Main where

import AdventLib (checkResult)
import Data.Functor (void)
import Data.Maybe (fromMaybe)
import qualified Data.Text as T
import qualified Data.Text.IO as T.IO
import Data.Void
import Text.Megaparsec
import Text.Megaparsec.Char
import Text.Megaparsec.Char.Lexer as L

-- import Text.Megaparsec.Debug

type Parser = Parsec Void T.Text

data Color = Color Int Int Int deriving (Show)

data GameRound = GameRound {colors :: [Color], gameId :: Int}
  deriving (Show)

pColor :: Parser Color
pColor = do
  n <- L.decimal
  void $ char ' '
  name <- string "red" <|> string "green" <|> string "blue"
  pure $ case name of
    "red" -> Color n 0 0
    "green" -> Color 0 n 0
    "blue" -> Color 0 0 n
    _ -> error "unreachable"

addColor :: Color -> Color -> Color
addColor (Color a1 b1 c1) (Color a2 b2 c2) = Color (a1 + a2) (b1 + b2) (c1 + c2)

defaultColor :: Color
defaultColor = Color 0 0 0

pHand :: Parser Color
pHand =
  let sep = void $ string ", "
      end = void $ string "; " <|> eol
   in do
        c1 <- pColor
        void $ optional sep
        c2 <- fromMaybe defaultColor <$> optional pColor
        void $ optional sep
        c3 <- fromMaybe defaultColor <$> optional pColor
        end
        pure $ foldr addColor defaultColor [c1, c2, c3]

pGameRound :: Parser GameRound
pGameRound = do
  _ <- chunk "Game " :: Parser T.Text
  gameId <- read <$> some digitChar
  void (string ": ")
  colors <- many pHand
  pure GameRound {colors, gameId}

possibleRound :: Color -> GameRound -> Bool
possibleRound (Color c11 c12 c13) gameRound =
  let c = map (\(Color c21 c22 c23) -> (c11 >= c21) && (c12 >= c22) && (c13 >= c23)) (colors gameRound)
   in and c

neededForRoundToBePossible :: GameRound -> Color
neededForRoundToBePossible gameRound =
  foldr (\(Color x1 y1 z1) (Color x2 y2 z2) -> Color (max x1 x2) (max y1 y2) (max z1 z2)) defaultColor $ colors gameRound

pRounds :: Parser [GameRound]
pRounds = do
  parsed <- many pGameRound
  void eof
  pure parsed

useParser :: ([GameRound] -> Int) -> FilePath -> IO Int
useParser f path = do
  contents <- T.IO.readFile path
  pure $ case parse pRounds path contents of
    Left errBundle -> error (errorBundlePretty errBundle)
    Right parsed -> f parsed

part1 :: FilePath -> IO Int
part1 = useParser f
  where
    f rounds = sum $ map gameId $ filter (possibleRound expectedColor) rounds
      where
        expectedColor = Color 12 13 14

part2 :: FilePath -> IO Int
part2 = useParser f
  where
    f rounds = sum $ map ((\(Color x y z) -> x * y * z) . neededForRoundToBePossible) rounds

main :: IO ()
main = do
  checkResult "part1 test" 8 $ part1 "inputtest.txt"
  checkResult "part1" 2348 $ part1 "input.txt"
  checkResult "part2 test" 2286 $ part2 "inputtest.txt"
  checkResult "part2" 76008 $ part2 "input.txt"
