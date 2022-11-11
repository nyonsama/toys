/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";

import { useEffect, useState } from "react";
import { Col, Container, Row } from "react-bootstrap";
import { Card, CardBody, CardTitle } from "../components/Card";
import Footer from "../components/Footer";
import Navbar from "../components/Navbar";
import { fetchHistory, HistoryData } from "../lib/api";

const HistoryCard = (props: { data: HistoryData }) => {
  const { data } = props;
  const keys: Array<[keyof HistoryData, string]> = [
    ["md5", "MD5"],
    ["apt", "是否为APT"],
    ["res", "检测结果"],
    ["filetype", "文件类型"],
    ["filesize", "文件大小"],
    // ["date", "检测日期"],
  ];
  return (
    <Card className="text-break">
      <CardTitle className="d-flex pt-3">
        {/* <div className="flex-grow-1 me-4">MD5: {data.md5}</div> */}
        <div>{data.date}</div>
      </CardTitle>
      <CardBody className="d-flex">
        <ul className="flex-grow-1 mb-0 ps-4">
          {keys.map(([k, desc]) => (
            <li
              key={k}
              css={css`
                margin: 0.25rem 0;
                /* padding: 0.25rem 0; */
                /* border-bottom: 1px solid #0000003f; */
              `}
            >
              <div className="d-flex">
                <div className="flex-grow-1">{desc}:</div>
                <div>{data[k]}</div>
              </div>
            </li>
          ))}
        </ul>
      </CardBody>
    </Card>
  );
};

const History = () => {
  const [history, setHistory] = useState<HistoryData[]>([]);
  useEffect(() => {
    fetchHistory().then((v) => {
      setHistory(v);
    });
  }, []);

  const cards = history.map((data, i) => {
    return (
      <Col lg={6} xxl={4}>
        <HistoryCard key={i} data={data} />
      </Col>
    );
  });

  return (
    <>
      <Navbar />
      <Container className="flex-grow-1 mb-4">
        <p className="fs-1 fw-light mt-2">历史记录</p>
        <hr />
        <Row style={{ rowGap: "1.5rem" }}>{cards}</Row>
      </Container>
      <Footer />
    </>
  );
};

export default History;
