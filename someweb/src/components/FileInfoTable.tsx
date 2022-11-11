/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";
import { FileInfo } from "../lib/api";

const fileInfoFieldName: [keyof FileInfo, string][] = [
  ["filename", "文件名"],
  ["sha256", "SHA256"],
  ["md5", "MD5"],
  ["filetype", "文件类型"],
  ["filesize", "文件大小"],
  ["date", "分析时间"],
  ["res", "检测结果"],
];
export const FileInfoTable = (props: { data?: FileInfo }) => {
  const { data } = props;
  const rows = data
    ? fileInfoFieldName
        .filter(([k]) => data[k] !== null && data[k] !== undefined)
        .map(([k, desc], i) => {
          return (
            <tr key={`${i}${k}`}>
              <td>{desc}</td>
              <td>{data[k]}</td>
            </tr>
          );
        })
    : null;
  return (
    <table
      css={css`
        table-layout: fixed;
        width: 100%;
        overflow-wrap: break-word;
        & td {
          padding: 0 0 0.25rem 0;
        }

        & > tbody > tr > td:first-of-type {
          color: #1572e8;
          width: 6rem;
        }

        & > tbody > tr > td:nth-of-type(2) {
          color: #575962;
          /* font-weight: 300; */
        }
      `}
    >
      <tbody>{rows}</tbody>
    </table>
  );
};
