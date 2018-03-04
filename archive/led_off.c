#include <stdio.h>
#include <stdlib.h>

#define FILE_TO_CONTROL     "/proc/acpi/ibm/led"
#define CONTROL_SEQ         "0 on\n"
#define CONTROL_LEN         5


void bail(char* errmsg, unsigned short bailcode) {
    fprintf(stderr, "%s\n", errmsg);
    exit(bailcode);
}


int main() {

    FILE* led_control = NULL;

    led_control = fopen(FILE_TO_CONTROL, "w");
    if (!(led_control)) {
        bail("fopen did bad!", 1);
    }

    size_t erramt = fwrite(CONTROL_SEQ,
                           sizeof(char),
                           CONTROL_LEN,
                           led_control);
    if (erramt < CONTROL_LEN) {
        bail("fwrite did bad!", 2);
    }

    fclose(led_control);
    return 0;

}
