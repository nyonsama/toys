/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";
import styled from "@emotion/styled";
import { ReactNode } from "react";

import architecture from "../architecture.png";
import extraction from "../gene_extraction.png";
import webFe from "../webfe.png";
import storage from "../storage.png";

const Vr = styled.hr`
  width: 1px;
  height: 100% !important;
  margin: 0 1rem;
`;

interface SubSectionProps {
  imgSrc: string;
  children?: ReactNode;
  reversed?: boolean;
}

const SubSection = ({ imgSrc, children, reversed }: SubSectionProps) => {
  const img = <img className="h-100 p-3" src={imgSrc} alt="Gene extraction" />;
  const desc = (
    <div className="d-flex flex-column align-items-center flex-grow-1">
      <div
        className="d-flex flex-column align-items-center"
        css={css`
          max-width: 32rem;
        `}
      >
        {children}
      </div>
    </div>
  );
  const Box = styled.div`
    display: flex;
    align-items: center;
    height: 20rem;
  `;
  if (reversed) {
    return (
      <Box>
        {img}
        <Vr />
        {desc}
      </Box>
    );
  } else {
    return (
      <Box>
        {desc}
        <Vr />
        {img}
      </Box>
    );
  }
};

const ImplementationSection = () => {
  return (
    <div
      className="container"
      css={css`
        & > div {
          margin-bottom: 1rem;
        }
      `}
    >
      <span className="fs-3">系统架构</span>
      <hr />
      <SubSection imgSrc={architecture}>
        <span className="fs-3 mb-2">三大模块</span>
        <ul>
          <li>基因提取</li>
          <li>APT检测平台</li>
          <li>Web前端</li>
        </ul>
      </SubSection>

      <SubSection imgSrc={extraction} reversed>
        <span className="fs-3 mb-2">基因提取部分</span>
        <p className="mx-4">
          第一部分基因提取部分，包括反汇编，软件脱壳以及基因提取、基因去噪，在这一部分本项目将程序最基本的功能块──基因提取出来，并以基因图谱形式放入下一部分即家族聚类中。
        </p>
      </SubSection>

      <SubSection imgSrc={storage}>
        <span className="fs-3 mb-2">APT检测平台</span>
        <p className="mx-4">
          第二部分是系统的核心即APT的检测平台。在这一部分包含对APT的检测模型、完成对APT的家族聚类和同源判定，并将检测结果传递给下一级Web端进行实时展示。
        </p>
      </SubSection>

      <SubSection imgSrc={webFe} reversed>
        <span className="fs-3 mb-2">Web前端</span>
        <p className="mx-4">
          第三部分是Web端的检测成果展示。这一部分将向用户展示APT基因图谱，并提供图谱中家族的基本介绍。在此基础上我们还允许用户上传其待测软件，帮助我们更进一步的完善APT基因图谱。
        </p>
      </SubSection>
    </div>
  );
};
export default ImplementationSection;
