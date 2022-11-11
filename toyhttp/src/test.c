#include "simplehttp.h"
#include "tool.h"
#include <stdio.h>
#include <sys/wait.h>
#include <syslog.h>
#include <unistd.h>

int main(int argc, char const *argv[]) {
  openlog(NULL, LOG_PERROR, LOG_USER);
  start_server(7733);
  return 0;
}
