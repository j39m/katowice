/* avocado_ackbar is a suid replacement
 * for guava_gundam.sh */

#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>


#define MAX_STRLEN      (65)
#define BRIGHTNESS_DIR  ("/sys/class/backlight/intel_backlight/")
#define MAXBRIGHT       ("max_brightness")
#define BRIGHT          ("brightness")

#define DRAT(k, r)        if (k) { return r; }


long int
getArg(int argc, char* argv[]) {
    long int argument;

    DRAT((argc != 2), 0);
    argument = strtol(argv[1], NULL, 10);

    return argument;
}


long int
readBrightness (const char* fileToRead) {
    FILE* fileObj       = NULL;
    char* fileContent   = NULL;
    size_t unused       = 0;
    long int retval     = 0;

    /* open the file */
    fileObj = fopen(fileToRead, "r");
    DRAT((!fileObj), 0);

    /* read and parse the file */
    getline(&fileContent, &unused, fileObj);
    retval = strtol(fileContent, NULL, 10);
    free(fileContent);
    fclose(fileObj);

    return retval;
}


unsigned int
commitBrightness (const char* brightnessFile, long int brightness) {
    FILE* fileObj       = NULL;
    char* commitBuf     = NULL;

    /* convert our arg into a string */
    commitBuf = calloc(sizeof(char), MAX_STRLEN);
    DRAT(!commitBuf, 1);
    snprintf(commitBuf, MAX_STRLEN, "%ld\n", brightness);

    /* open the file for writing */
    fileObj = fopen(brightnessFile, "w");
    DRAT(!fileObj, 1);

    /* perform the write */
    fwrite(commitBuf, sizeof(char), strlen(commitBuf), fileObj);

    /* clean up */
    free(commitBuf);
    fclose(fileObj);

    return 0;
}


int main(int argc, char* argv[]) {

    long int    changeAmount    = 0;
    long int    currBright      = 0;
    long int    maxBright       = 0;
    char*       currBrightFile  = NULL;
    char*       maxBrightFile   = NULL;
    unsigned int result         = 0;

    /* read how much we want to change by */
    changeAmount = getArg(argc, argv);
    DRAT(!changeAmount, 1);

    /* prepare to read in current settings */
    currBrightFile = calloc(sizeof(char), MAX_STRLEN);
    maxBrightFile = calloc(sizeof(char), MAX_STRLEN);
    DRAT(!(currBrightFile && maxBrightFile), 1);
    snprintf(currBrightFile, MAX_STRLEN,
             "%s%s", BRIGHTNESS_DIR, BRIGHT);
    snprintf(maxBrightFile, MAX_STRLEN,
             "%s%s", BRIGHTNESS_DIR, MAXBRIGHT);

    /* read in current settings */
    currBright = readBrightness(currBrightFile);
    maxBright = readBrightness(maxBrightFile);
    DRAT(!maxBright, 1);

    /* what's our new setting? */
    currBright += changeAmount;
    if (currBright < 0) {
        currBright = 0;
    } else if (currBright > maxBright) {
        currBright = maxBright;
    }

    /* commit to new brightness */
    result = commitBrightness(currBrightFile, currBright);
    DRAT(result, 1);

    free(currBrightFile);
    free(maxBrightFile);

    return 0;

}
