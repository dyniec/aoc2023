{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.Maybe (fromMaybe)
import Data.Text (Text)
import Data.Text.IO (getContents)
import Data.Void
import Text.Megaparsec
import Text.Megaparsec.Char (char, newline, string)
import Text.Megaparsec.Char.Lexer (decimal)
import Prelude hiding (getContents)

type Parser = Parsec Void Text

data Color = Red | Green | Blue
  deriving (Show, Eq)

data Hand = Hand Integer Integer Integer
  deriving (Show, Eq)

data Game = Game Integer [Hand]
  deriving (Show, Eq)

countColors :: [(Color, Integer)] -> Hand
countColors xs = Hand (get xs Red) (get xs Green) (get xs Blue)
  where
    get :: [(Color, Integer)] -> Color -> Integer
    get xs c = fromMaybe 0 (lookup c xs)

instance Ord Hand where
  (Hand a b c) <= (Hand a1 b1 c1) = (a <= a1) && (b <= b1) && (c <= c1)

color :: Parser Color
color = do
  (string "red" >> pure Red) <|> (string "blue" >> pure Blue) <|> (string "green" >> pure Green)

numColor :: Parser (Color, Integer)
numColor = do
  num <- decimal
  char ' '
  col <- color
  pure (col, num)

hand :: Parser Hand
hand = do
  countColors <$> sepBy numColor (string ", ")

game :: Parser Game
game = do
  string "Game "
  game_num <- decimal
  string ": "
  hands <- sepBy hand (string "; ")
  pure $ Game game_num hands

wholeInput :: Parser [Game]
wholeInput = do
  games <- sepEndBy game newline
  eof
  pure games

isGameOk :: Game -> Bool
isGameOk (Game _ xs) = all (<= Hand 12 13 14) xs

gameId (Game i _) = i

main :: IO ()
main = do
  input <- getContents
  let (Right games) = runParser wholeInput "/dev/stdin" input
  let res = sum$map gameId $ filter isGameOk games
  print res
