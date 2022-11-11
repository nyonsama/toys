#include "netutil.h"

int parse_ip(const char *ip, uint16_t port, struct sockaddr *addr) {
  if (!strchr(ip, ':')) {
    struct sockaddr_in *v4addr = addr;
    v4addr->sin_family = AF_INET;
    v4addr->sin_port = htons(port);
    if (inet_pton(AF_INET, ip, &v4addr->sin_addr) == 0) {
      return 1;
    }
  } else {
    struct sockaddr_in6 *v6addr = addr;
    v6addr->sin6_family = AF_INET6;
    v6addr->sin6_port = htons(port);
    if (inet_pton(AF_INET6, ip, &v6addr->sin6_addr) == 0) {
      return 1;
    }
  }
  return 0;
}

int tcp_listen(const char *ip, uint16_t port) {
  struct sockaddr addr = {};
  if (parse_ip(ip, port, &addr) != 0) {
    FATAL("Invalid address\n");
    exit(EXIT_VALUE_INET_PTON);
  }

  int fd = socket(addr.sa_family, SOCK_STREAM, (int)NULL);
  if (fd == -1) {
    FATAL(strerror(errno));
    exit(EXIT_VALUE_SOCKET);
  }

  if (bind(fd, &addr, sizeof(struct sockaddr_in6)) == -1) {
    FATAL(strerror(errno));
    close(fd);
    exit(EXIT_VALUE_BIND);
  }

  if (listen(fd, TCP_LISTEN_QUEUE_LEN) == -1) {
    FATAL(strerror(errno));
    close(fd);
    exit(EXIT_VALUE_LISTEN);
  }
  return fd;
}

int tcp_connect(const char *ip, uint16_t port) {
  struct sockaddr addr = {};
  parse_ip(ip, port, &addr);

  int fd = socket(addr.sa_family, SOCK_STREAM, (int)NULL);
  if (fd == -1) {
    FATAL(strerror(errno));
    exit(EXIT_VALUE_SOCKET);
  }
  if (connect(fd, &addr, sizeof(struct sockaddr_in6)) == -1) {
    FATAL(strerror(errno));
    close(fd);
    exit(EXIT_VALUE_CONNECT);
  }
  return fd;
}

ssize_t tcp_recv(int sfd, void *data, ssize_t maxlen, ssize_t bufsize) {
  ssize_t size = 0;
  char *buf = malloc(bufsize);
  data = malloc(bufsize);
  for (int i = 0;; i++) {
    int count = recv(sfd, buf, bufsize, 0);
    size += count;
    if (count == -1) {
      perror("tcp_recv error");
      free(buf);
      return -1;
    } else if (size == maxlen) {
      break;
    } else if (count == bufsize) {
      data = realloc(data, size + bufsize);
    } else {
      data = realloc(data, size);
      break;
    }
  }
  free(buf);
  return size;
}