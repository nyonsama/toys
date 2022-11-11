#include "tool.h"

/**
 * @brief convert ip string and port number to struct sockaddr
 * 
 * @param ip 
 * @param port 
 * @param addr 
 * @return 0 on success, -1 on failure
 */
int gen_sockaddr(const char *ip, uint16_t port, struct sockaddr *addr) {
  struct sockaddr_in *v4addr = (struct sockaddr_in *)addr;
  v4addr->sin_family = AF_INET;
  v4addr->sin_port = htons(port);
  if (inet_pton(AF_INET, ip, &v4addr->sin_addr) == 0) {
    syslog(LOG_ERR, "Failed to parse IP address \"%s\"", ip);
    return -1;
  }
  return 0;
}

/**
 * @brief listen tcp connections on ip:port
 * 
 * @param ip c string
 * @param port 
 * @return int (socket fd) or -1 on failure 
 */
int tcplis(const char *ip, uint16_t port) {
  struct sockaddr addr = {};
  if (gen_sockaddr(ip, port, &addr) != 0) {
    return -1;
  }

  int fd = socket(addr.sa_family, SOCK_STREAM, 0);
  if (fd == -1) {
    syslog(LOG_ERR, "Failed to initialize socket: %m");
    return -1;
  }

  if (bind(fd, &addr, sizeof(struct sockaddr_in)) == -1) {
    syslog(LOG_ERR, "Failed to bind socket on %s:%d: %m", ip, port);
    close(fd);
    return -1;
  }

  if (listen(fd, TCP_LISTEN_QUEUE_LEN) == -1) {
    syslog(LOG_ERR, "Failed to listen: %m");
    close(fd);
    return -1;
  }
  return fd;
}

/**
 * @brief start a tcp connection to ip:port
 * 
 * @param ip 
 * @param port 
 * @return int (socket fd) or -1 on failure
 */
int tcpconn(const char *ip, uint16_t port) {
  struct sockaddr addr = {};
  if (gen_sockaddr(ip, port, &addr) != 0) {
    return -1;
  };

  int fd = socket(addr.sa_family, SOCK_STREAM, 0);
  if (fd == -1) {
    syslog(LOG_ERR, "Failed to initialize socket: %m");
    return -1;
  }
  if (connect(fd, &addr, sizeof(struct sockaddr_in)) == -1) {
    syslog(LOG_ERR, "Failed to connect to socket: %m");
    close(fd);
    return -1;
  }
  return fd;
}
