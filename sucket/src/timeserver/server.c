#include "timeserver.h"

void log_info(const struct sockaddr *client_addr, FILE *logfile) {
  char buf[INET6_ADDRSTRLEN] = {};
  union sockaddr_union {
    struct sockaddr_in v4;
    struct sockaddr_in6 v6;
    struct sockaddr raw;
  };
  union sockaddr_union *addr = (union sockaddr_union *)client_addr;
  const char *ip;
  if (addr->raw.sa_family == AF_INET)
    ip = inet_ntop(AF_INET, &addr->v4.sin_addr, buf, INET_ADDRSTRLEN);
  else if (addr->raw.sa_family == AF_INET6)
    ip = inet_ntop(AF_INET6, &addr->v6.sin6_addr, buf, INET6_ADDRSTRLEN);
  time_t curtime;
  time(&curtime);
  fprintf(logfile, "%s %s", ip, ctime(&curtime));
  fflush(logfile);
}

int main(int argc, char const *argv[]) {
  int sfd = tcp_listen("127.0.0.1", 11945);
  printf("listening 127.0.0.1:%d\n", 11945);
  FILE *logfile = fopen("timeserver.log", "a");
  while (1) {
    struct sockaddr clientaddr = {};
    socklen_t addrlen = sizeof(clientaddr);
    int clientfd = accept(sfd, &clientaddr, &addrlen);
    if (clientfd == -1) {
      FATAL(strerror(errno));
      exit(1);
    }
    timepacket request;
    if (recv(clientfd, &request, sizeof(timepacket), 0) != sizeof(timepacket)) {
      fprintf(stderr, "recv error: Invalid length\n");
      FATAL(strerror(errno));
      exit(3);
    }
    if (memcmp(&request.header, HEADER_GET_TIME, sizeof(request.header)) == 0) {
      timepacket response = {.header = {}, .timestamp = time(NULL)};
      if (send(clientfd, &response, sizeof(timepacket), 0) == -1) {
        FATAL(strerror(errno));
        exit(2);
      }
      log_info(&clientaddr, logfile);
    } else {
      fprintf(stderr, "unsupported operation\n");
    }
  }
  return 0;
}
