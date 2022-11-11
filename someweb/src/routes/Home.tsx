/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";

import styled from "@emotion/styled";

import background from "../bg.jpg";

import Navbar from "../components/Navbar";
import UploadSection from "../components/UploadSection";
import IntroductionSection from "../components/IntroductionSection";
import ImplementationSection from "../components/ImplementationSection";
import AptFamilySection from "../components/AptFamilySection";
import Footer from "../components/Footer";
import { FileUploadProvider } from "../context/FileUploadContext";

const backgroundHeight = 4065;

const Background = styled.img`
  position: absolute;
  width: 100%;
  height: ${backgroundHeight}px;
  z-index: -1;
  object-fit: cover;
  object-position: top center;
`;

// todo 箭头动画 分页指示器 回到顶部

const Home = () => {
  return (
    <FileUploadProvider>
      <Navbar />
      <div className="position-relative">
        <Background src={background} />
        <div
          css={css`
            height: ${backgroundHeight}px;
            display: flex;
            flex-direction: column;
            & > * {
              flex: 1;
              color: white;
            }
          `}
        >
          <UploadSection />
          <IntroductionSection />
          <AptFamilySection />
          <ImplementationSection />
        </div>
      </div>
      <Footer />
    </FileUploadProvider>
  );
};

export default Home;
