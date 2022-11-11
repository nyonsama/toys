#include "fileserver.h"

// 返回目录列表，把文件数量写进count
// count可以为NULL
// 不要忘了free
// 测试过了，能用
char *listdir(const char *path, size_t *count) {
  DIR *dp;
  struct dirent *ep;

  char *filenames = calloc(1, 256);
  size_t _count = 0;

  dp = opendir(path);
  if (dp != NULL) {
    while (ep = readdir(dp)) {
      _count += 1;
      filenames = realloc(filenames, 257 * _count); //需要验证可行性
      strncat(filenames, ep->d_name, 256);
      strncat(filenames, "\n", 2);
    }
    closedir(dp);
  } else
    perror("Couldn't open the directory");
  if (count)
    *count = _count;
  return filenames;
}

void send_file(FILE *file, int sfd) {
  // 发送文件内容
  while (!feof(file)) {
    char buf[1024] = {};
    size_t size = fread(buf, 1, 1024, file);
    if (send(sfd, buf, size, 0) == -1) {
      FATAL(strerror(errno));
    }
  }
  fclose(file);
}
// 遇到错误会返回-1
off_t fsize(const char *filename) {
  struct stat st;

  if (stat(filename, &st) == 0)
    return st.st_size;

  return -1;
}
