#ifndef __DEBUG_H__
#define __DEBUG_H__

#include <execinfo.h>
#include <stdio.h>

// format可以为空
#define FATAL(format, ...)                                                     \
  fprintf(stderr, "A fatal error occurred. Stack trace:\n");                   \
  void *sbuf[20];                                                              \
  int depth = backtrace(sbuf, sizeof(sbuf));                                   \
  backtrace_symbols_fd(sbuf, depth, STDERR_FILENO);                            \
  fprintf(stderr, "\n%s:%d: ", __FILE__, __LINE__ - 1);                        \
  fprintf(stderr, format, ##__VA_ARGS__);                                      \
  putc('\n', stderr)

// 程序的返回值

#define EXIT_VALUE_SOCKET 1
#define EXIT_VALUE_INET_PTON 2
#define EXIT_VALUE_BIND 3
#define EXIT_VALUE_LISTEN 4
#define EXIT_VALUE_ACCEPT 5
#define EXIT_VALUE_CONNECT 6
#define EXIT_VALUE_RECV 7

#endif