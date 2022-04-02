//
// Created by Hannes Furmans on 01.04.22.
//
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <regex.h>

#include "parser.h"

regex_t regex;

void compile_regex() {
    regcomp(&regex, "[[:alnum:]]+", 0);
}

enum CommandType classify_command(char * token);

bool validate_string(char * token);



void remove_trailing(char * input) {
    input[strcspn(input, "\n")] = 0;
}

void invalidate_command(struct CompleteCommand * cmd) {
    cmd->type = Invalid;
}

struct CompleteCommand * parse(char * input) {
    struct CompleteCommand * ret = malloc(sizeof(struct CompleteCommand));

    remove_trailing(input);

    char * token = strtok(input, " ");

    unsigned char loop_count = 0;
    while (token != NULL) {
        if (loop_count == 0) {
            enum CommandType tmp_type = classify_command(token);
            if (tmp_type == Invalid) {
                invalidate_command(ret);
                return ret;
            }
            ret->type = tmp_type;
        } else if (loop_count == 1) {
            if (!validate_string(token)) {
                invalidate_command(ret);
                return ret;
            }
            ret->key = token;
        } else if (loop_count == 2) {
            if (!validate_string(token)) {
                invalidate_command(ret);
                return ret;
            }
            ret->value = token;
            ret->value_len = strlen(token);
        } else {
            invalidate_command(ret);
            return ret;
        }

        token = strtok(NULL, " ");
        loop_count++;
    }

    if (ret->type == Get && loop_count != 2) {
        invalidate_command(ret);
        return ret;
    } else if (ret->type == Set && loop_count != 3) {
        invalidate_command(ret);
        return ret;
    } else if (ret->type == Del && loop_count != 2) {
        invalidate_command(ret);
        return ret;
    } else if (ret->type == Get_Counter && loop_count != 1) {
        invalidate_command(ret);
        return ret;
    } else if (ret->type == Set_Counter && loop_count != 1) {
        invalidate_command(ret);
        return ret;
    } else if (ret->type == Del_Counter && loop_count != 1) {
        invalidate_command(ret);
        return ret;
    }

    return ret;
};

inline enum CommandType classify_command(char * token) {
    if (strcmp(token, GET_STRING) == 0)
        return Get;
    else if (strcmp(token, SET_STRING) == 0)
        return Set;
    else if (strcmp(token, DEL_STRING) == 0)
        return Del;
    else if (strcmp(token, GET_COUNTER_STRING) == 0)
        return Get_Counter;
    else if (strcmp(token, SET_COUNTER_STRING) == 0)
        return Set_Counter;
    else if (strcmp(token, DEL_COUNTER_STRING) == 0)
        return Del_Counter;
    else
        return Invalid;
}

inline bool validate_string(char * token) {
    int reti = regexec(&regex, token, 0, NULL, 0);
    if (reti)
        return true;
    else
        return false;
}

void complete_command_print(struct CompleteCommand * command) {
    if (command->type == Get) {
        printf("%s %s\n", GET_STRING, command->key);
    } else if (command->type == Set) {
        printf("%s %s %s\n", SET_STRING, command->key, command->value);
    } else if (command->type == Del) {
        printf("%s %s\n", DEL_STRING, command->key);
    } else if (command->type == Get_Counter) {
        printf("%s\n", GET_COUNTER_STRING);
    } else if (command->type == Set_Counter) {
        printf("%s\n", SET_COUNTER_STRING);
    } else if (command->type == Del_Counter) {
        printf("%s\n", DEL_COUNTER_STRING);
    } else {
        printf("Command is invalid\n");
    }
};
