/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";

import { Container } from "react-bootstrap";

import introduction from "../attract.png";

export default function IntroductionSection() {
  return (
    <Container className="d-flex flex-column">
      <span className="fs-3">APT家族</span>
      <hr />
      <div className="d-flex flex-column align-items-center">
        <img
          src={introduction}
          alt=""
          css={css`
            width: 32rem;
            padding-bottom: 1rem;
          `}
        />
        <p>
          cxxz同一个家族的不同APT都有着一些相似性和共用的特点。同时，一些不同家族APT之间也测再有千丝万缕的联系。如图所示，属同一家族APT(相同颜色点)之间存在联系(边)的同时，不同家族之间也存在有相当多的联系。
        </p>
      </div>
    </Container>
  );
}
