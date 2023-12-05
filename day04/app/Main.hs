{-# LANGUAGE OverloadedStrings #-}

module Main where

import Control.Monad (void)
import Data.List (intersect)
import Data.Maybe (fromMaybe)
import Data.Text (Text)
import Data.Text.IO (getContents)
import Data.Void
import Text.Megaparsec
  ( MonadParsec (eof),
    Parsec,
    endBy,
    runParser,
    sepBy,
    sepEndBy,
    some,
  )
import Text.Megaparsec.Char (char, newline, string)
import Text.Megaparsec.Char.Lexer (decimal)
import Prelude hiding (getContents)

type Parser = Parsec Void Text

data Card = Card Integer [Integer] [Integer]
  deriving (Show, Eq)

card :: Parser Card
card = do
  void $ string "Card"
  whitespace
  num <- decimal
  void $ char ':'
  whitespace
  hand <- endBy decimal whitespace
  void $ char '|'
  whitespace
  winning <- sepBy decimal whitespace
  pure $ Card num hand winning

whitespace :: Parser ()
whitespace = void $ some (char ' ')

wholeInput :: Parser [Card]
wholeInput = do
  cards <- sepEndBy card newline
  eof
  pure cards

score :: Card -> Int
score (Card _ hand winning) = len
  where
    len = length shared
    shared = hand `intersect` winning

go :: [(Int, Integer)] -> Integer -> Integer
go [] a = a
go ((score, repeats) : xs) a = go addRepeats  (a + repeats)
  where
    addRepeats :: [(Int, Integer)]
    addRepeats  = map (\(a, b) -> (a, b + repeats)) (take score xs) ++ drop score xs

main :: IO ()
main = do
  input <- getContents
  let parseResult = runParser wholeInput "/dev/stdin" input
  print parseResult
  let (Right cards) = parseResult
  print cards
  let res = go (map ((, 1) . score) cards) 0
  print res
