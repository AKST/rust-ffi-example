#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <inttypes.h>


/* external */
typedef struct Email Email;

Email *email_parse(int32_t, char *);
void email_display(Email *);
void email_free(Email *);


/* internal */
typedef long long ll;

int with_file_lines(FILE *, bool (*)(int32_t, char *));
bool with_line(int32_t, char *);


int main(int argc, char **args) {
    if (argc < 2) return 1;
    const char *fname = args[1];
    FILE *emails = fopen(fname, "r");
    if (emails == NULL) return 1;
    return with_file_lines(emails, with_line);
}


int with_file_lines(FILE *file, bool (*operation)(int32_t, char *)) {
    size_t size  = 0;
    ssize_t read = 0;
    char *buffer = NULL;
    bool last_op = true;
    while (last_op && (read = getline(&buffer, &size, file)) > 0) {
        //buffer[read - 1] = '\0';
        last_op = operation(read, buffer);
    }
    fclose(file);
    return last_op ? 0 : 1;
}


bool with_line(int32_t size, char *line) {
    Email *email;
    if (line && (email = email_parse(size, line))) {
        email_display(email);
        email_free(email);
        return true;
    }
    else {
        printf("unable to parse\n");
        return false;
    }
}
