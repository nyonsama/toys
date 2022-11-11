/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";

import styled from "@emotion/styled";
import { Button, Modal } from "react-bootstrap";
import { BsChevronDoubleDown } from "react-icons/bs";

import UploadButton from "../components/UploadButton";
import { useFileUpload } from "../context/FileUploadContext";

const Title = styled.p`
  color: white;
  font-weight: 300;
  text-align: center;
  font-size: 3rem;
  margin-bottom: 4rem;
`;

const UploadSection = () => {
  const uploadFile = useFileUpload();
  return (
    <>
      <div
        css={css`
          /* flex: 1; */
          padding-top: 280px;
        `}
      >
        <div
          css={css`
            animation: fadein 0.6s cubic-bezier(0, 0, 0.2, 1) 0.1s;
            /* animation: name duration timing-function delay iteration-count direction fill-mode; */
            animation-fill-mode: both;
            @keyframes fadein {
              from {
                transform: scale(0.7);
                opacity: 0;
              }
              to {
                opacity: 100;
              }
            }
          `}
          className="d-flex flex-column align-items-center"
        >
          <Title>基于基因图谱的APT攻击检测与同源判定系统</Title>
          <UploadButton style={{ marginBottom: "4rem" }} onClick={uploadFile}>
            上传文件
          </UploadButton>
          <p className="text-white">阅读更多</p>
          {/* TODO 给箭头加动画 */}
          <BsChevronDoubleDown size="4rem" color="white" />
        </div>
      </div>
    </>
  );
};

export default UploadSection;
