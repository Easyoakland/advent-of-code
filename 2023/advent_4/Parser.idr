
module Parser
import Types

import Data.String
import Control.Monad.State
import Control.Monad.Trans

%default total

||| Parser
||| @i input
||| @o output
||| @e error
public export Parser : (i, o, e : Type) -> Type
Parser i o e = StateT i (Either e) o

public export Parser' : (o: Type) -> Type
Parser' o = Parser String o String

export
Functor Parser' where
    map f a = a >>= \a => pure (f a)
export
Applicative Parser' where
    pure = pure
    (<*>) f a = f >>= \f => map f a
export
[instAlternativeParser'] Alternative Parser' where
    empty = lift $ Left "empty"
    (<|>) x y = do
        i <- get
        let Left err_x = runStateT i x
            | Right (i', x) => do put i'; pure x
        let Left err_y = runStateT i y
            | Right (i', y) => do put i'; pure y
        lift $ Left $ interpolate "Alternatives failed: \{err_x}, \{err_y}"

export
Alternative Parser' where
    empty = empty @{instAlternativeParser'}
    (<|>)= (<|>) @{instAlternativeParser'}

-- WTH does the `Alternative Parser'` instance not get detected without this? :(
-- It should be literally the same thing. Clearly implicit search is bugged or I don't understand how it's supposed to work.
-- The unamed version is also shown above and somehow it's not detected.
export
Alternative (StateT String (Either String)) where
    empty = empty {f=Parser'}
    (<|>)= (<|>) {f=Parser'}

-- I think it has something to do with function extensionality/equality not working in Idris based on the following where all thm typecheck except thm4
-- thm1 : Parser' o = Parser String o String
-- thm1 = Refl
-- thm2 : Parser String o String = StateT String (Either String) o
-- thm2 = Refl
-- thm3 : Parser' o  = StateT String (Either String) o
-- thm3 = Refl
-- thm4 : Parser' = StateT String (Either String) -- This doesn't work even though its just an eta contracted version of the previous
-- thm4 = Refl

||| Checks predicate on first element of str
peek_str : String -> (f: Char -> Bool) -> Bool
peek_str str f = case asList str of
    c :: _ => f c
    [] => False

is_digit : Char -> Bool
is_digit c = c >= '0' && c <= '9'

||| Checks if next char is a digit
peek_is_digit : Parser' Bool
peek_is_digit = do
    i <- get
    pure $ case asList i of
        c :: i => is_digit c
        _ => False

||| Given two lists @a and @b, return the longest common prefix and the two suffixes
export
lcp : Eq t => List t -> List t -> (List t, List t, List t)
lcp a b = lcp_aux a b [] where
    lcp_aux : List t -> List t -> List t -> (List t, List t, List t)
    lcp_aux a [] pref = (pref, a, [])
    lcp_aux [] b pref = (pref, [], b)
    lcp_aux (a::as) (b::bs) pref = if a /= b then (pref, a::as, b::bs) else
        let (pref, a_suf, b_suf) = lcp_aux as bs pref in
        (a::pref, a_suf, b_suf)

||| Parse prefix
lit : String -> Parser' $ String
lit str = do
    i <- get
    let (pref, i', _) = lcp (unpack i) (unpack str)
    if length pref /= length str then do lift $ Left $ interpolate "expected \{str} not \{!get}"
        else do
            put $ pack i'
            pure $ pack pref

||| Parses next char if it satisfies a predicate
char_with : (Char -> Bool) -> Parser' $ Maybe Char
char_with f = do
    i <- get
    let StrCons c i = strM i
        | _ => pure Nothing
    if f c then
     do put i; pure $ Just c
     else pure Nothing

export
char : Char -> Parser' $ Char
char c = do
    res <- char_with (== c)
    case res of
        Just c => lift $ Right c
        Nothing => lift $ Left $ interpolate "expected \{show c} not \{!get}" -- !expr is like x <- expr; x
                                                                              -- Lean uses `(<- expr)` for this instead

||| Parse a positive number
posNum : Num a => Parser' a
posNum = do
    i <- get
    let (pre, suf) = String.span is_digit i
    let Just num = parsePositive pre
        | Nothing => lift $ Left "not a digit" -- Idris has a nice early return syntax which looks like let else from Rust. :)
    put suf -- remove the parsed prefix on success
    pure num

covering export
||| Consume whitespace
ws : Parser' String
ws = ws_aux "" where
    ws_single = char ' ' <|> char '\n' <|> char '\r'
    ws_aux : String -> Parser' String
    ws_aux acc = do
        i <- get
        let Right (i', c) = runStateT i ws_single
            | Left err => pure acc
        put i'
        acc <- ws_aux acc
        pure $ strCons c acc

covering public export
list : Parser' item -> Parser' sep -> Parser' $ List item
list p_item p_sep = do -- More compilated but equivalent: num_list = mapStateT (>>= \(state, list) => Right (state, reverse list)) $ num_list_aux [] where
    val <- list_aux []
    pure $ reverse val
    where
    list_aux : List item -> Parser' $ List item
    list_aux acc = do
        i <- get

        -- Get a number or finish
        let Right (i, num) = runStateT i p_item
            | _ => pure acc
        put i

        -- Skip seperator
        _ <- p_sep

        -- Note that even though `acc` is used linearly this function can't be linear on acc because List (and specifically `(::)`) doesn't support linear usage.
        -- This is itself because Idris doesn't have linearity parametricity.
        -- This goes to show that linear types are significantly less useful if things aren't by-default linear (or linear parametric).
        -- Also lacking affinity in the std library makes many would-be uses of linearity not possible.
        -- :(
        list_aux (num :: acc)

export covering
card : Parser' Card
card = do
    _ <- lit "Card" *> ws
    _ <- posNum {a=Nat}
    _ <- lit ":" *> ws
    winning_nums <- list posNum ws
    _ <- lit "|" *> ws
    have_nums <- list posNum ws

    pure (Card.Mk {winning_nums, have_nums})

export covering
parse : String -> Either String $ List Card
parse str = evalStateT str $ list card ws