#include "fileserver.h"
// listen
//分析请求

int main(int argc, char const *argv[])
{
  int sfd = tcp_listen("127.0.0.1", 14852);

  printf("listening 127.0.0.1:%d\n", 14852);
  while (1)
  {
    struct sockaddr clientaddr = {};
    socklen_t addrlen = sizeof(clientaddr);
    int clientfd = accept(sfd, &clientaddr, &addrlen);
    if (clientfd == -1)
    {
      FATAL(strerror(errno));
      exit(1);
    }
    req_hdr request;
    if (recv(clientfd, &request, sizeof(req_hdr), 0) != sizeof(req_hdr))
    {
      fprintf(stderr, "recv error: Invalid header length\n");
      FATAL(strerror(errno));
      exit(3);
    }

    request.filename[256] = 0; // waf
    switch (request.operation)
    {
    case REQ_GET:
    {

      FILE *file = fopen(request.filename, "br");
      if (file)
      {

        // 发送响应头
        resp_hdr response = {
            .status = RESP_OK,
            .info = {},
            .size = fsize(request.filename)}; // TODO 对fsize的错误处理
        send(clientfd, &response, sizeof(resp_hdr), 0);

        // 发送文件内容
        while (!feof(file))
        {
          char buf[1024] = {};
          size_t size = fread(buf, 1, 1024, file);
          send(clientfd, buf, size, 0);
        }
        fclose(file);
      }
      else
      {
        // 打开文件错误
        resp_hdr response = {.status = RESP_ERROR, .info = {}, .size = 0};
        strncpy(response.info, strerror(errno), sizeof(response.info));
        send(clientfd, &response, sizeof(resp_hdr), 0);
      }
      break;
    }
    case REQ_LIST:
      break;
    case REQ_STORE:
    { // TODO 现在对话还不完整
      FILE *file = fopen(request.filename, "wb");
      size_t size = 0;

      // 接收文件内容
      char buf[1024] = {};
      do
      {
        // TODO 加入超时取消接收的机制，避免长时间阻塞
        size = recv(clientfd, buf, 1024, 0);
        if (size == -1)
        {
          FATAL(strerror(errno));
        }
        fwrite(buf, 1, size, file);
        if (ferror(file))
        {
          fprintf(stderr, "write file error\n");
          break; // TODO
                 // 处理好写入出错的情况，让客户端知道出错，防止客户端被长时间阻塞
        }
      } while (size != 0 && size <= request.size);
      fclose(file);

      resp_hdr response = {.status = RESP_OK, .info = {}, .size = 0};
      break;
    }
    default:
      // response.status = RESP_ERROR;
      break;
    }
    /*
    if (send(clientfd, &response, sizeof(resp_hdr), 0) == -1) {
      FATAL(strerror(errno));
      exit(2);
    }
    fprintf(stderr, "unsupported operation\n");
    */
  }
  return 0;
}
