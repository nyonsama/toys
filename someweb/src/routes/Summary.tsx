/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";

import styled from "@emotion/styled";
import { useEffect, useState } from "react";
import { Container } from "react-bootstrap";

import { buildStyles, CircularProgressbar } from "react-circular-progressbar";
import "react-circular-progressbar/dist/styles.css";
import { Card, CardBody, CardTitle } from "../components/Card";
import { FileInfoTable } from "../components/FileInfoTable";
import Footer from "../components/Footer";

import Navbar from "../components/Navbar";
import Vr from "../components/Vr";
import {
  FileUploadProvider,
  useFileUpload,
} from "../context/FileUploadContext";
import {
  AnalyzeResult,
  FeatureDetail,
  fetchAnalyzeResult,
  fetchCurrentFileInfo,
  fetchFeatureDetail,
  FileInfo,
  saveCurrentFileInfo,
} from "../lib/api";

const MaliciousnessProgressBar = (props: {
  value: number;
  color: string;
  label: string;
}) => {
  return (
    <div style={{ maxWidth: "12rem" }}>
      <div
        css={css`
          position: relative;
        `}
      >
        <CircularProgressbar
          value={props.value}
          styles={buildStyles({
            pathColor: props.color,
            textColor: "black",
          })}
        />
        <div
          css={css`
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            font-size: 1.5rem;
          `}
        >
          {props.value}%
        </div>
      </div>
      <div className="text-center mt-2">{props.label}</div>
    </div>
  );
};

const ScoreCard = () => {
  const [result, setResult] = useState<AnalyzeResult>();
  useEffect(() => {
    fetchAnalyzeResult().then((v) => {
      setResult(v);
    });
  }, []);
  const colors = ["rgb(43, 185, 48)", "#3e98c7", "rgb(242, 89, 97)"];

  return (
    <Card>
      <CardTitle>
        Maliciousness:
        {result?.detect === "apt" ? (
          <span className="text-danger">APT</span>
        ) : (
          <span className="text-success">Not APT</span>
        )}
      </CardTitle>
      <CardBody>
        <div
          css={css`
            margin-top: 1rem;
          `}
          className="d-flex justify-content-around gap-4"
        >
          {Object.entries(result?.classify ?? {}).map(([k, v], i) => (
            <MaliciousnessProgressBar
              key={i}
              value={(v as number) * 100}
              color={colors[i]}
              label={k}
            />
          ))}
        </div>
      </CardBody>
    </Card>
  );
};

const FileInfoCard = (props: { data?: FileInfo }) => {
  const data = props.data;
  return (
    <Card>
      <CardTitle>基本信息 </CardTitle>
      <CardBody>
        <FileInfoTable data={data} />
      </CardBody>
    </Card>
  );
};

const RoundedButton = styled.button`
  border: 1px solid #ffffffcf;
  border-radius: 50rem;
  color: white;
  background-color: transparent;
  text-align: center;
  vertical-align: center;
  padding: 0.375rem 1.25rem;
  @keyframes hover {
    to {
      background-color: rgb(88, 103, 221);
    }
  }
  &:hover {
    animation: hover 0.2s ease-in-out;
    animation-fill-mode: both;
  }
`;

const fadeIn = css`
  animation: fadein 0.2s;
  animation-fill-mode: both;
  @keyframes fadein {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
`;
const fadeOut = css`
  user-select: none;
  animation: fadeout 0.2s;
  animation-fill-mode: both;
  @keyframes fadeout {
    from {
      opacity: 1;
    }
    to {
      opacity: 0;
    }
  }
`;
const Header = () => {
  const [showToast, setShowToast] = useState(false);
  const uploadFile = useFileUpload();

  const handleClickSave = async () => {
    await saveCurrentFileInfo();
    setShowToast(true);
    setTimeout(() => {
      setShowToast(false);
    }, 2000);
  };
  return (
    <>
      <div
        className="text-white"
        css={css`
          background: linear-gradient(-45deg, #06418e, #1572e8);
          height: 12rem;
        `}
      >
        <div
          style={{ paddingTop: "2.5rem" }}
          className="d-flex justify-content-between container text-white"
        >
          <div>
            <p className="fs-1 fw-light">分析结果</p>
            {/* <span>Results given based on version of : 04/04/2020</span> */}
          </div>
          <div className="d-flex align-items-end gap-3 ">
            <RoundedButton onClick={handleClickSave}>Save</RoundedButton>
            <RoundedButton
              style={{ backgroundColor: "rgb(77, 90, 192)" }}
              onClick={uploadFile}
            >
              New Scan
            </RoundedButton>
          </div>
        </div>
      </div>
      <div
        style={{
          zIndex: 99,
          position: "fixed",
          bottom: "4rem",
          right: "4rem",
        }}
      >
        <div
          className="alert alert-success"
          style={{ width: "24rem" }}
          css={showToast ? fadeIn : fadeOut}
        >
          保存成功
        </div>
      </div>
    </>
  );
};

const FeatureDetails = () => {
  const [featureDetail, setFeatureDetail] = useState<FeatureDetail>();
  useEffect(() => {
    fetchFeatureDetail().then((v) => {
      setFeatureDetail(v);
    });
  }, []);

  const gene = featureDetail
    ? featureDetail.gene[0].gene.map((v, i) => <li key={i}>{v}</li>)
    : null;

  const vector = featureDetail
    ? featureDetail.vector[0].vector.join(", ")
    : null;

  return (
    <Card>
      <CardTitle className="d-flex justify-content-between">
        Feature details
      </CardTitle>
      <CardBody className="d-flex justify-content-between">
        <div style={{ width: "calc(50% - 1rem)" }}>
          <div className="fs-5 mb-2">Gene</div>
          <ul
            style={{ height: "24rem", overflowY: "scroll" }}
            className="list-unstyled mb-0"
          >
            {gene}
          </ul>
        </div>
        <Vr className="mx-3" />
        <div style={{ width: "calc(50% - 1rem)" }}>
          <div className="fs-5 mb-2">Vector</div>
          <div style={{ height: "24rem", overflowY: "scroll" }}>{vector}</div>
        </div>
      </CardBody>
    </Card>
  );
};

const Summary = () => {
  const [fileInfo, setFileInfo] = useState<FileInfo>();
  useEffect(() => {
    fetchCurrentFileInfo().then((v) => {
      setFileInfo(v);
    });
  }, []);

  return (
    <FileUploadProvider>
      <Navbar />
      <Header />
      <Container
        className="flex-grow-1 d-flex flex-column"
        css={css`
          position: relative;
          top: -3rem;
          gap: 2rem;
        `}
      >
        <div
          className="d-flex"
          css={css`
            gap: 2rem;
            & > * {
              flex: 1;
            }
          `}
        >
          <ScoreCard />
          <FileInfoCard data={fileInfo} />
        </div>
        <FeatureDetails />
      </Container>
      <Footer />
    </FileUploadProvider>
  );
};

export default Summary;
