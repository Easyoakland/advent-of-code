module Main

-- https://idris2docs.sinyax.net/
-- for Idris std lib documentation

-- LSP doesn't support renaming :(
-- LSP randomly crashes :(
-- LSP randomly doesn't show the type of things and never shows the type left of the <- in do notation unless the left binder is a hole :(

import Types -- Idris doesn't have selective imports and namespace's don't allow re-exporting imports to emulate them. :(
import Parser
import Control.Monad.State
import Control.Monad.Trans
import Control.Monad.Error.Either
import Control.Monad.Error.Interface
import System.File
import Data.Zippable
import Prelude.Basics
import Data.List -- This is another pain point of Idris. If an implementation exists, it is hard to find it because the tooling doesn't help at all. The online docs don't even let you find where the implementation it claims to know of exists. In this case I can't go to definition for `Data.List` but it clearly has the List instances such as Zippable List. :(
import Data.Nat
import Data.SortedSet
import Data.SortedMap
import Data.List.Lazy -- This doesn't exist in most recent release
import Debug.Trace
import Data.Maybe

dbg : Show a => a -> a
dbg a = trace (show a) a
-- dbg a = a

winning_numbers_cnt : Card -> Nat
winning_numbers_cnt c = have_that_won where
    set_of_winners : SortedSet Nat
    set_of_winners = fromList c.winning_nums
    is_winner : Nat -> Bool
    is_winner n = contains n set_of_winners
    lazy_have_nums: LazyList Nat
    lazy_have_nums = fromList c.have_nums -- filter doesn't exist on `List` :( only LazyList
    count : LazyList a -> Nat
    count l = sum $ map (const {a=Nat} 1) l -- LazyList doesn't have a length or count method :(
    have_that_won = count $ filter is_winner lazy_have_nums

||| Part 1 worth of card
worth_of_card : Card -> Nat
worth_of_card c = if winning_numbers_cnt c == 0 then 0 else power 2 $ (winning_numbers_cnt c) `minus` 1

||| Merge two sets of card counts
||| Just like mergeWith just ignores elements only present in the right set
merge_card_list : SortedMap Nat Nat -> SortedMap Nat Nat -> SortedMap Nat Nat
merge_card_list = mergeWith (+)

range : Nat -> Nat -> List Nat -- Idris implements ranges with inclusive both ends. That's wrong. :(
range n m = if n >= m then [] else rangeFromTo n (m `minus` 1)

enumerate : List a -> List (Nat, a)
enumerate l = zip (range 0 $ length l) l

||| Give the list of cards, generate a map for each card of how many (recursive) occurrences of other cards are duplicated when this one is duplicated
generated_cumulative_card_cnt : List Card -> SortedMap Nat (SortedMap Nat Nat)
generated_cumulative_card_cnt cards = go (length cards) (reverse (cards)) (fromList []) -- reverse cards so the are last first
    where
        go : Nat -> (List Card) -> (SortedMap Nat (SortedMap Nat Nat)) -> (SortedMap Nat (SortedMap Nat Nat))
        go _ [] acc = acc
        go current_card (card::rest) acc =
            let to_insert = foldl -- foldl should be linear on x' but isn't, again Idris linearity is half-baked :(
                        (\acc, elem => merge_card_list elem acc) -- merge each into this set
                        (fromList $ map (\x => (x,1)) $ range (current_card+1) (current_card+1+win_cnt)) -- new entry for card's count starts with the 1-step immediate copies from this card
                        $ (map (\n => fromMaybe empty_set $ lookup n acc ) (range (current_card+1) (current_card+1+win_cnt))) -- accumulates all card counts from cards that are "duplicated"
                        in
            -- insert the cumulative counts of all lower cards copied when this card copies.
            let acc = insert current_card ( to_insert ) acc in
            go (current_card`minus`1) rest acc
        where
            win_cnt = winning_numbers_cnt card
            empty_set : SortedMap Nat Nat
            empty_set = fromList []

||| Total amount of card copies per card
total_cards : List Card -> SortedMap Nat Nat
total_cards cards =
    let extra_per_card_copy = generated_cumulative_card_cnt cards in
    foldl (\acc, (current_card, extra) => let extra = SortedMap.toList extra in
            let acc = incr_entry current_card 1 acc in -- add 1 count for self
             -- add count to other extra cards
            foldl (\acc, (extra_card, extra_count) => incr_entry extra_card extra_count acc) acc extra
        ) (fromList []) (SortedMap.toList extra_per_card_copy)
    where
    incr_entry : Nat -> Nat -> SortedMap Nat Nat -> SortedMap Nat Nat
    incr_entry k v map = update (
            \entry => let entry = fromMaybe 0 entry in Just $ entry + v
        ) k map

part1 : String -> Either String Nat
part1 str = do
    input <- parse str
    pure $ sum $ map worth_of_card input

part2 : String -> Either String Nat
part2 str = do
    input <- parse str
    pure $ sum {a=Nat} $ total_cards input

main : IO ()
main = do
    mainRes <- runEitherT fallible_main
    case mainRes of
        Left err => printLn $ interpolate "failed with error: \{err}"
        Right val => printLn "done"
    where
    fallible_main : EitherT String IO ()
    fallible_main = do
        Right test_input <- readFile "inputtest.txt"
            | Left err => left $ show err
        part1_test_res <- liftEither $ part1 test_input
        let True = part1_test_res == 13
            | False => left $ interpolate "part1 test fail, got \{show part1_test_res} not 13"

        Right input <- readFile "input.txt"
            | Left err => left $ show err
        part1_res <- liftEither $ part1 input
        let True = part1_res == 21158
            | False => left $ interpolate "part1 fail, got \{show part1_res} not 21158"

        printLn "part1 pass"

        part2_test_res <- liftEither $ part2 test_input
        let True = part2_test_res == 30
            | False => left $ interpolate "part1 test fail, got \{show part2_test_res} not 31"

        Right input <- readFile "input.txt"
            | Left err => left $ show err
        part2_res <- liftEither $ part2 input
        let True = part2_res == 6050769
            | False => left $ interpolate "part2 fail, got \{show part2_res} not 6050769"

        printLn "part2 pass"

-- main : IO ()
-- main = let r = runStateT "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53" card in case r of
--         Right (s, o) => printLn $ interpolate "`\{o}` then `\{s}`"
--         Left err => printLn $ interpolate "err `\{err}`"

-- main : IO ()
-- main = let r = runStateT "  \n 4 \n" (ws) in case r of
--         Right (s, o) => printLn $ interpolate "`\{o}` then `\{s}`"
--         Left err => printLn $ interpolate "err \{err}"

{-
||| Is this a docstring?
||| ```testlang
||| main
||| ```
||| Don't know how to build docs
main : IO ()
main = let r: Either String (String, List Nat) = (runStateT "1 2 3" Parser.num_list) in case r of
        Right (s, o) => printLn $ interpolate "\{show o} then \{s}"
        Left err => printLn $ interpolate "err \{err}" -}

-- main : IO ()
-- main = printLn $ show $ lcp [1,2,3,5] [1,4,2,7]

--- `interpolate` uses \{} like a format string