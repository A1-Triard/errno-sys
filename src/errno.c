#include <errno.h>

int *rust_errno_sys_errno_location() { return &errno; }
