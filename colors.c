/* colors.c
 * solution to Sanjeev's question about colors on a strip
 * with constraints
 */

#include <stdio.h>

// define limits. index colors from 1.
#define NUMCOLORS       (2)
#define CONSECUTIVES    (5)
#define LISTLEN         (13)


/* fill a list following the set down restraints -
 * no more than CONSECUTIVES colors in a row.
 * returns the number of combos possible under given constraints.
 */
unsigned int
fillList(unsigned int prevColor,
         unsigned int inARow,
         unsigned int lenRemain) {

    unsigned int colorIter;
    unsigned int result;

    if (!lenRemain) {
        return 1;
    }

    result = 0;
    for (colorIter=1; colorIter<=NUMCOLORS; ++colorIter) {
        if (colorIter == prevColor) {
            if (inARow == CONSECUTIVES) {
                continue;
            }
            result += fillList(colorIter, inARow+1, lenRemain-1);
        } else {
            result += fillList(colorIter, 1, lenRemain-1);
        }
    }

    return result;
}

int main(void) {

    unsigned int result;
    result = fillList(0, 0, LISTLEN);
    printf("result on %d colors, %d consecutives, and "
           "list len %d is %d\n",
           NUMCOLORS, CONSECUTIVES, LISTLEN, result);
    return 0;
}
