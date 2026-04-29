#include <stddef.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <eva/abi.h>

void kputs(const char *str) {
    int len = 0;
    while(str[len] != 0)
        len += 1;
    eva_c_io_kwrite((const uint8_t*)str, len);
}

void *__wrap__malloc_r(void *reent, size_t n_bytes) {
    (void)reent;
    return eva_c_emu_malloc(n_bytes);
}

void __wrap__free_r(void *reent, void *data) {
    (void)reent;
    eva_c_emu_free(data);
}

int _fstat(int fd, struct stat *buf) {
    buf->st_mode = S_IFCHR;        /* Always pretend to be a tty */
    buf->st_blksize = 0;
    return 0;
}

int _isatty(int fd) {
    (void)fd;
    return 1;
}

int _read(int fd, void *ptr, size_t len) {
    return eva_c_io_kread(ptr, len);
}

int _write(int fd, const char *ptr, size_t len) {
    return eva_c_io_kwrite(ptr, len);
}

int _close(int fd) {
    (void)fd;
    return -1;
}

off_t _lseek(int fd, off_t offset, int whence) {
    (void)fd;
    (void)offset;
    (void)whence;
    return ((off_t)-1);
}

int _kill(int pid, int sig) {
    (void)pid;
    (void)sig;
    eva_c_rt_abort();
    return -1;
}

int _getpid() {
    return 0;
}

void _exit(int status) {
    (void)status;
    eva_c_rt_abort();
}