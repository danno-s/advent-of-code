import Data.List
import Data.Hash.MD5

root = "iwrupvqb"

main = putStr $ show $ fst $ head $ filter (\x -> replicate 6 '0' `isPrefixOf` snd x) [ (i, md5s (Str $ root ++ show i)) | i <- [1..] ]
