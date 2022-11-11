#ifndef __FILESERVER_H__
#define __FILESERVER_H__

#include "netutil.h"
#include <dirent.h>
#include <libgen.h>
#include <stdint.h>
#include <sys/stat.h>
//#include <sys/types.h>

#define REQ_LIST 0
#define REQ_GET 1
#define REQ_STORE 2

#define RESP_OK 0
#define RESP_ERROR 1
//#define RESP_NOTFOUND 3
//#define RESP_DENIED 4

typedef struct {
  uint8_t operation;
  char username[16];
  char password[32];
  char filename[256];
  uint64_t size; //接下来要传输的数据大小
} req_hdr;

typedef struct {
  uint8_t status;
  char info[32];
  uint64_t size; //要往回传的数据大小
} resp_hdr;

off_t fsize(const char *filename);
void send_file(FILE *file, int sfd);
char *listdir(const char *path, size_t *count);

#endif