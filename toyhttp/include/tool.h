#ifndef __TOOL_H__
#define __TOOL_H__

#include "debug.h"
#include <arpa/inet.h>
#include <errno.h>
#include <netinet/in.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <syslog.h>
#include <unistd.h>

// The number of connection requests will be queued before further requests are
// refused.
// 如果等待处理的请求数量达到这个值，后来的请求会被拒绝
#define TCP_LISTEN_QUEUE_LEN 10

int tcplis(const char *ip, uint16_t port);
int tcpconn(const char *ip, uint16_t port);

int gen_sockaddr(const char *ip, uint16_t port, struct sockaddr *addr);

#endif