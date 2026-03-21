#define _GNU_SOURCE
#include <stdio.h>
#include <stddef.h>


static void show_buf(const char *label, FILE *f)
{
    struct _IO_FILE *fp = (struct _IO_FILE *)f;
    ptrdiff_t buf_total = fp->_IO_buf_end   - fp->_IO_buf_base;
    ptrdiff_t pending   = fp->_IO_write_ptr - fp->_IO_write_base;
    ptrdiff_t free_left = fp->_IO_write_end - fp->_IO_write_ptr;
    fprintf(stderr, "[%s] buf: %td filled, %td free, %td total\n", label, pending, free_left, buf_total);
}

int main(void)
{
    static char buf[12];
    setvbuf(stdout, buf, _IOFBF, sizeof buf);
    show_buf("after setvbuf", stdout);
    printf("lazy ");
    show_buf("after printf(\"lazy \")", stdout);
    printf("hello ");
    show_buf("after printf(\"hello \")", stdout);
    printf("world ");
    show_buf("after printf(\"world \")", stdout);
    printf("buffered!\n");
    show_buf("after printf(\"buffered!\\n\")", stdout);
    fflush(stdout);
    show_buf("after fflush", stdout);
    return 0;
}
