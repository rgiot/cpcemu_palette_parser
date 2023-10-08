#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Return NULL if success and a pointer to a string in case of error. You have to dealloc the string yourself.
 * Line must contain only utf8 valid chars
 */
const int8_t *cpcemu_execute_line(const int8_t *line);

void launch_tests(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
