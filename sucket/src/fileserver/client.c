#include "fileserver.h"

void print_usage(const char *myname) {
  const char *usage = "simple file backup client\n"
                      "usage: %s ip port filename\n";
  printf(usage, myname);
  return;
}

void store_file(const char *ip, uint16_t port, char *filepath) {
  int serverfd = tcp_connect(ip, port);

  // 发送请求头
  req_hdr request = {.operation = REQ_STORE,
                     .filename = {},
                     .size = fsize(filepath)}; // TODO 对fsize返回-1的处理

  FILE *file = fopen(filepath, "rb");

  //怪东西，可能会修改filepath的内容，所以把fopen放在上面
  const char *filename = basename(filepath);

  strncpy(request.filename, filename, sizeof(request.filename));
  if (send(serverfd, &request, sizeof(req_hdr), 0) == -1) {
    FATAL(strerror(errno));
  }

  if (file) {
    send_file(file, serverfd);
  } else {
    FATAL("read file error");
  }
}

int main(int argc, char const *argv[]) {
  if (argc != 4) {
    print_usage(argv[0]);
    exit(255);
    return 255;
  }
  char filepath[PATH_MAX] = {};
  strncpy(filepath, argv[3], PATH_MAX);
  store_file(argv[1], atoi(argv[2]), filepath);
  return 0;
}
