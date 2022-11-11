#include "timeserver.h"

void print_usage(const char *myname) {
  const char *usage = "simple time client\n"
                      "usage: %s ip port\n";
  printf(usage, myname);
  return;
}

time_t ask_time(const char *ip, uint16_t port) {

  timepacket request = {.header = {}, .timestamp = 0};
  memcpy(&request.header, HEADER_GET_TIME, sizeof(request.header));

  int sfd = tcp_connect(ip, port);
  if (send(sfd, &request, sizeof(timepacket), 0) == -1) {
    perror("ask_time send");
  }

  timepacket response;
  int recvlen = recv(sfd, &response, sizeof(timepacket), 0);
  if (recvlen != sizeof(timepacket)) {
    fprintf(stderr, "invalid length\n");
    perror("ask_time recv");
    return -1;
  }
  return response.timestamp;
}

int main(int argc, char const *argv[]) {
  if (argc != 3) {
    print_usage(argv[0]);
    exit(255);
    return 255;
  }
  const time_t timestamp = ask_time(argv[1], atoi(argv[2]));
  if (timestamp == -1) {
    print_usage(argv[0]);
  } else {
    printf("server time: %s", ctime(&timestamp));
  }
  return 0;
}
