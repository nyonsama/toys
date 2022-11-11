#include "simplehttp.h"

void resp_headers(int sfd, int status, const char *header) {
  char *buf = calloc(1, 512);
  if (header == NULL) {
    header = "";
  }
  switch (status) {
  case 200:
    snprintf(buf, 512, "%s\r\n%s\r\n\r\n", "HTTP/1.1 200 OK", header);
    break;
  case 404:
    snprintf(buf, 512, "%s\r\n%s\r\n\r\n404", "HTTP/1.1 404 Not Found", header);
    break;
  case 500:
    snprintf(buf, 512, "%s\r\n%s\r\n\r\n500",
             "HTTP/1.1 500 Internal Server Error", header);
    break;
  }
  write(sfd, buf, strnlen(buf, 512));
  free(buf);
}

void resp_file(int sfd, const char *filename, const char *header) {
  size_t bufsize = 512;
  char *buf = calloc(1, bufsize);
  snprintf(buf, bufsize, "./html/%s", filename);
  int fd = open(buf, O_RDONLY);
  if (fd < 0) {
    syslog(LOG_ERR, "Failed to open %s: %m", filename);
    resp_headers(sfd, 500, header);
  } else {
    write(sfd, "HTTP/1.1 200 OK\r\n\r\n", 19);
    sendfile(sfd, fd, NULL, 1 << 30);
    close(fd);
  }
  free(buf);
}

void resp_cgi(int sfd, const char *filename, const char *query) {
  size_t bufsize = 512;
  char *buf = calloc(1, bufsize);
  if (query != NULL) {
    snprintf(buf, bufsize, "./html/%s %s", filename, query);
  } else {
    snprintf(buf, bufsize, "./html/%s", filename);
  }
  FILE *p = popen(buf, "r");
  if (p == NULL) {
    syslog(LOG_ERR, "Failed to run %s: %m", buf);
    resp_headers(sfd, 500, NULL);
  } else {
    resp_headers(sfd, 200, "");
    memset(buf, 0, bufsize);
    while (!feof(p)) {
      fread(buf, bufsize, 1, p);
      write(sfd, buf, strlen(buf));
    }
  }
  free(buf);
  pclose(p);
}

void resp_ls(int sfd, const char *filename) {
  size_t bufsize = 512;
  char *buf = calloc(1, 512);
  snprintf(buf, bufsize, "LANG=C ls --file-type ./html/%s 2>&1", filename);
  FILE *p = popen(buf, "r");
  if (p == NULL) {
    syslog(LOG_ERR, "Failed to run %s: %m", buf);
    resp_headers(sfd, 500, "");
  } else {
    memset(buf, 0, bufsize);
    resp_headers(sfd, 200, "");
    while (!feof(p)) {
      fread(buf, bufsize, 1, p);
      write(sfd, buf, strlen(buf));
    }
    pclose(p);
  }
}

struct request *parse_request(const char *header, size_t len,
                              struct request *req) {
  size_t i = 0;
  size_t last = i;
  for (; i < len && header[i] != ' '; i++) {
    req->method[i] = header[i];
  }
  i++;
  last = i;

  for (; i < len; i++) {
    if (header[i] == ' ' || header[i] == '?') {
      size_t len = i - last;
      req->path = strncpy(malloc(len + 1), &header[last], len);
      req->path[len] = '\0';
      break;
    }
  }
  i++;
  last = i;

  if (header[i - 1] == '?') {
    for (; i < len; i++)
      if (header[i] == ' ') {
        req->query = strncpy(malloc(i - last + 1), &header[last], i - last);
        req->query[i - last] = '\0';
        break;
      }
  }
  return req;
}

void free_request(struct request *request) {
  free(request->path);
  free(request->query);
}

void serve(int sfd) {
  size_t bufsize = 8192;
  char *buf = malloc(bufsize);
  size_t len = read(sfd, buf, bufsize - 1);
  if (len < 0) {
    syslog(LOG_ERR, "Failed to read from socket: %m");
    close(sfd);
  } else if (len == 0) {
    syslog(LOG_WARNING, "pid%d: Empty request", getpid());
    close(sfd);
  } else {
    // printf("%s\n", buf);
    buf[len] = 0;
    struct request req = {};
    parse_request(buf, len, &req);
    if (strcmp(req.method, "GET") == 0) {
      if (strcmp(req.path, "/") == 0) {
        resp_file(sfd, "index.html", "");
      } else if (strstr(req.path, ".cgi") - req.path == strlen(req.path) - 4) {
        resp_cgi(sfd, req.path, req.query);
      } else if (req.path[strlen(req.path) - 1] == '/') {
        resp_ls(sfd, req.path);
      } else {
        resp_file(sfd, req.path, "");
      }
    }
    free_request(&req);
  }
  free(buf);
}

static void /* SIGCHLD handler to reap dead child processes */
grimReaper(int sig) {
  int savedErrno; /* Save 'errno' in case changed here */

  savedErrno = errno;
  while (waitpid(-1, NULL, WNOHANG) > 0)
    continue;
  errno = savedErrno;
}

void start_server(uint16_t port) {
  struct sigaction sa;
  sigemptyset(&sa.sa_mask);
  sa.sa_flags = SA_RESTART;
  sa.sa_handler = grimReaper;
  if (sigaction(SIGCHLD, &sa, NULL) == -1) {
    syslog(LOG_ERR, "Error from sigaction(): %m");
    exit(EXIT_FAILURE);
  }

  int listen_fd = tcplis("0.0.0.0", port);
  if (listen_fd < 0) {
    syslog(LOG_ERR, "Failed to listen: %m");
    return;
  };
  while (1) {
    struct sockaddr_in addr = {};
    socklen_t addrlen = sizeof(addr);
    int sfd = accept(listen_fd, (struct sockaddr *)&addr, &addrlen);
    if (sfd < 0) {
      syslog(LOG_ERR, "Failed to accept socket: %m");
      return;
    }
    char logbuf[128] = {};
    uint16_t remote_port = ntohs(addr.sin_port);
    inet_ntop(AF_INET, &addr.sin_addr, logbuf, sizeof(logbuf));
    syslog(LOG_INFO, "starting connection from %s:%d", logbuf, remote_port);
#if defined(SIMPLEHTTP_FORK)
    pid_t pid = fork();
    if (pid == 0) {
      serve(sfd);
      close(sfd);
      exit(0);
    } else if (pid > 0) {
      close(sfd);
    } else {
      syslog(LOG_ERR, "Failed to fork: %m");
    }
    syslog(LOG_INFO, "stopping connection from %s:%d", logbuf, remote_port);
#else
    serve(sfd);
    close(sfd);
#endif // SIMPLEHTTP_FORK
  }
}
