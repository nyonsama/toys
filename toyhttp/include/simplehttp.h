#ifndef __SIMPLEHTTP_H__
#define __SIMPLEHTTP_H__

#include "tool.h"
#include <fcntl.h>
#include <regex.h>
#include <signal.h>
#include <stdint.h>
#include <string.h>
#include <sys/sendfile.h>
#include <syslog.h>
#include <unistd.h>
#include <wait.h>

struct request {
  char method[8];
  char *path;
  char *query;
};

struct request *parse_request(const char *header, size_t len,
                              struct request *req);
void free_request(struct request *request);
void serve(int sfd);
void start_server(uint16_t port);
void resp_file(int sfd, const char *filename, const char *header);
void resp_cgi(int sfd, const char *filename, const char *query);
void resp_ls(int sfd, const char *dir);
void resp_headers(int sfd, int status, const char *header);

#define SIMPLEHTTP_FORK

#endif