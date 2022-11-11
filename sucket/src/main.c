#include <arpa/inet.h>
#include <errno.h>
#include <execinfo.h>
#include <netinet/in.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <unistd.h>

/*
// format可以为空
#define FATAL(format, ...)                                                     \
  fprintf(stderr, "A fatal error occurred. Stack trace:\n");                   \
  void *sbuf[20];                                                              \
  int depth = backtrace(sbuf, sizeof(sbuf));                                   \
  backtrace_symbols_fd(sbuf, depth, STDERR_FILENO);                            \
  fprintf(stderr, "\n%s:%d: " format, __FILE__, __LINE__ - 1, ##__VA_ARGS__);
*/

#include "netutil.h"
/*
int main(int argc, char const *argv[]) {
  const char *ip = "127.0.0.1";
  uint16_t port = 19280;

  int fd = socket(AF_INET, SOCK_STREAM, (int)NULL);
  if (fd == -1) {
    FATAL();
    perror(NULL);
    exit(1);
  }

  struct sockaddr_in addr = {.sin_family = AF_INET,
                             .sin_port = htons(port),
                             .sin_addr = 0,
                             .sin_zero = {}};
  if (inet_pton(AF_INET, ip, &addr.sin_addr) == -1) {
    FATAL("Invalid address\n");
    exit(2);
  }

  if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) == -1) {
    FATAL();
    perror(NULL);
    exit(3);
  }

  if (listen(fd, 1) == -1) {
    FATAL();
    perror(NULL);
    exit(4);
  }

  struct sockaddr_in client_addr = {};
  socklen_t addr_len = sizeof(client_addr);
  int conn_fd = accept(fd, (struct sockaddr *)&client_addr, &addr_len);
  if (conn_fd == -1) {
    FATAL();
    perror(NULL);
    exit(5);
  }

  char buf[100] = {};
  ssize_t len = recv(conn_fd, buf, 100, (int)NULL);
  if (len == -1) {
    FATAL();
    perror(NULL);
    exit(6);
  }
  printf("%s\n", buf);
  FATAL();
  perror(NULL);
  return 0;
}
*/

int main(int argc, char const *argv[]) {
  /*
  struct sockaddr client_addr = {};
  socklen_t addr_len = sizeof(client_addr);
  int fd = tcp_listen("::1", 19280);
  int conn_fd = accept(fd, &client_addr, &addr_len);
  if (conn_fd == -1) {
    perror(NULL);
    close(fd);
    exit(5);
  }
  */

  //char buf[100] = {};
  const char* buf="asdfzxcv";
  int fd = tcp_connect("127.0.0.1", 19280);
  ssize_t len = send(fd, buf, 9, (int)NULL);
  if (len == -1) {
    perror(NULL);
    exit(6);
  }
  printf("%s", buf);
}