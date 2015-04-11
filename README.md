# katowice 

... is a potpourri of projects I cobble together in my spare time. They have no coherence, no point, no taste. What else could they be for but to stroke my ego? 

## guava_gundam 
... is a bash script that performs the extremely hacky way of adjusting my screen brightness, by parsing in the current value (via ACPI), doing janky arithmetic with it, and writing it back. 

## pear_management
... is a little C library in which I experiment with creating more accessible lists in C. It's really practice with zero-length arrays, but I think lists of primitives need not muck with linked lists or such. 

## tomato_enpassant
... is a bash script to automate the repetitive stuff I usually have to do to get a dual-screen setup working on Openbox. The usage is pretty restrictive, but it works beautifully for my particular setup. 

## sorts 
... is practice writing Scala. I've implemented some basic things (at time of writing, Mergesort and Quicksort) and will add more as time goes on. 

## unskipper 
... is a Python script that prunes skipcounts from your Quod Libet library. It's a selfish tic of mine that I don't like seeing nonzero skipcounts on my songs because usually they happen by accident, and of course increase proportionally with the number of times I listen to a song (probably nonlinearly). It'll zero out all skipcounts in your library by popping the associated key from the dictionaries in your library and write the results back to disk. 

## emuberry_elgar & durian_elgar
... are a pair of Perl scripts that do "absolutely nothing". emuberry_elgar will take a sequence of entirely lowercase words (no punctuation; delimit with the string "STOP") and print a transformation of each word to stdout, for which the format will be "(int wordlength).(cryptographic hash of the word)," for which the dot is a literal period (not just silent concatenation). The particular hash defaults to md5, but can be changed by tweaking the appropriate parameters in both scripts (they must match for the scripts to work - in particular, be sure that you have the correct $hashlen variable in the latter, should you change to a different hash). durian_elgar will take in a message having the same form of output as emuberry_elgar and print the message that corresponds to the input hash. Beware: there is absolutely no memoization (to be implemented), no rainbow tables, no nothing - so durian_elgar will run horrendously slow in practice, unless u sv rly trn ct ea ch wrd to be de cd ed. 

## orange_curtain
... is a Python script that solves Puzzle no. 100 from "Professor Layton and the Curious Village." It (currently) unwisely iterates across all 28! (there isn't an SI prefix to ballpark this, I'm afraid) possible solutions to determine which one is valid. 

## grape_chair 
... is a Python script that solves Puzzle no. 95 from "Professor Layton and the Curious Village." Of the 720 possible answers this script proves that the solution is unique. For the effort that went into this script the result is unfortunately not impressive, given that Deng 2015 solved the puzzle in two-ish minutes by intuition. 

## peanut_airstrip
... is a Perl script that tries to extract JPEG images from very certain webpages. This is a one-off script that I threw together in a hurry, it's not meant to be flexible. 

## raspberry_meatballs

... is a Perl script that looks for the absence of "cover" files in my Music directory. It does not perform a serious recursive traversal: it goes exactly one level down into the subdirectories (of argument 1) and no further, looking for the absence of a file (specified by argument 2). My use is for seeking out albums in which I once encoded the album art into the FLAC files directly, which is behavior I now wish to revers. 

## rhubarb_lemonade 

... is a Perl script that counts word frequency in my quantum computing term paper. It takes one mandatory and one optional argument; a file and a discard threshold on frequency, respectively. So, for example, invoking `./rhubarb_lemonade klaus 5` tells you how often each word in the file `klaus` appears, sorted in descending numerical frequency. All words that appear five or fewer times in `klaus` are left out of the accounting. 

## ubans 

... is a Python script that permutes words. Specifically, running 

`ubans(word)` 

prints out all the permutations of the word argument. The print statements are all tiled twice across, because this script came about as a result of curiosity over how else I could express "snabu snabu" besides "ubans ubans." 

## chicken_salad

... is a Python script that does a number of little things for which I need infrequently and don't want to continuously consult WolframAlpha about. An example is the `expm()` function, which performs modular exponentiation based on its arguments `base, exponent, modulo`. 
