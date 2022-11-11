#ifndef __NETUTIL_H__
#define __NETUTIL_H__

#include "debug.h"
#include <arpa/inet.h>
#include <errno.h>
#include <netinet/in.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

// The number of connection requests will be queued before further requests are
// refused.
// 如果等待处理的请求数量达到这个值，后来的请求会被拒绝
#define TCP_LISTEN_QUEUE_LEN 10

int tcp_listen(const char *ip, uint16_t port);
int tcp_connect(const char *ip, uint16_t port);

ssize_t tcp_recv(int sfd, void *data, ssize_t maxlen, ssize_t bufsize);

// 成功则返回0，ip格式不对就返回1
int parse_ip(const char *ip, uint16_t port, struct sockaddr *addr);

#endif