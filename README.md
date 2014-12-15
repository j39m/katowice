# katowice 

... is a potpourri of projects I cobble together in my spare time. They have no coherence, no point, no taste. What else could they be for but to stroke my ego? 

## rhubarb_lemonade 

... is a Perl script that counts word frequency in my quantum computing term paper. It takes one mandatory and one optional argument; a file and a discard threshold on frequency, respectively. So, for example, invoking `./rhubarb_lemonade klaus 5` tells you how often each word in the file `klaus` appears, sorted in descending numerical frequency. All words that appear five or fewer times in `klaus` are left out of the accounting. 

## ubans 

... is a Python script that permutes words. Specifically, running 

`ubans(word)` 

prints out all the permutations of the word argument. The print statements are all tiled twice across, because this script came about as a result of curiosity over how else I could express "snabu snabu" besides "ubans ubans." 

## chicken_salad

... is a Python script that does a number of little things for which I need infrequently and don't want to continuously consult WolframAlpha about. An example is the `expm()` function, which performs modular exponentiation based on its arguments `base, exponent, modulo`. 
