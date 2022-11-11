#ifndef __TIME_SERVER_H__
#define __TIME_SERVER_H__

#include "netutil.h"
#include <stdbool.h>
#include <stdint.h>
#include <sys/socket.h>
#include <time.h>

const char HEADER_GET_TIME[8] = {'g', 'e', 't', 0, 0, 0, 0, 0};

typedef struct {
  char header[8];
  uint64_t timestamp;
} timepacket;

time_t ask_time(const char *ip, uint16_t port);

#endif