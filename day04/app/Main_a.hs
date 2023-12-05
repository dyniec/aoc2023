{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.Maybe (fromMaybe)
import Data.Text (Text)
import Data.Text.IO (getContents)
import Data.Void
import Text.Megaparsec
    ( MonadParsec(eof), Parsec, endBy, sepBy, sepEndBy, runParser,some)
import Text.Megaparsec.Char (char, newline, string)
import Text.Megaparsec.Char.Lexer (decimal)
import Prelude hiding (getContents)
import Data.List (intersect)
import Control.Monad (void)

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
whitespace = void $ some ( char ' ')

wholeInput :: Parser [Card]
wholeInput = do
  cards <- sepEndBy card newline
  eof
  pure cards

score :: Card -> Integer
score (Card _ hand winning)= if len ==0 then 0 else 2^ (len-1)
  where
    len = length shared
    shared = hand `intersect` winning

main :: IO ()
main = do
  input <- getContents
  let parseResult = runParser wholeInput "/dev/stdin" input
  print parseResult
  let (Right cards) = parseResult
  print cards
  let res = sum $ map score cards
  print res
