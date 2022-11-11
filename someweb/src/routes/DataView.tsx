/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";

import { Col, Container, Row } from "react-bootstrap";
import Navbar from "../components/Navbar";

import { Card, CardBody, CardTitle } from "../components/Card";
import styled from "@emotion/styled";
import Footer from "../components/Footer";
import Vr from "../components/Vr";
import React, { useEffect, useState } from "react";
import {
  fetchAptCount,
  fetchMd5TableData,
  PieData,
  pieDataUrl,
} from "../lib/api";
import { BsLink45Deg } from "react-icons/bs";

import { Chart, registerables } from "chart.js";
import { Pie } from "react-chartjs-2";
Chart.register(...registerables);

const data02 = [
  { name: "A1", value: 100 },
  { name: "A2", value: 300 },
  { name: "B1", value: 100 },
  { name: "B2", value: 80 },
  { name: "B3", value: 40 },
  { name: "B4", value: 30 },
];

interface PieCardProps {
  label: string;
  data: { name: string; value: number }[];
}

const PieCard = (props: PieCardProps) => {
  const colors = props.data.map(
    (v, i, a) => `hsl(${(360 / a.length) * i}, 70%, 50%)`
  );
  // const colors = [
  //   "#F66D44",
  //   "#FEAE65",
  //   "#E6F69D",
  //   "#AADEA7",
  //   "#64C2A6",
  //   "#2D87BB",
  // ];
  return (
    <Card className="d-flex flex-column">
      <CardTitle>{props.label}</CardTitle>
      <CardBody className="flex-grow-1 d-flex justify-content-center">
        <div className="h-100">
          <Pie
            options={{
              responsive: true,
              plugins: {
                legend: {
                  position: "bottom",
                },
              },
            }}
            data={{
              labels: props.data.map((v) => v.name),
              datasets: [
                {
                  data: props.data.map((v) => v.value),
                  backgroundColor: colors,
                },
              ],
            }}
          />
        </div>
      </CardBody>
    </Card>
  );
};
const BasicInfo = () => {
  const ListRow = (props: {
    name: string;
    value: string | number;
    className?: string;
  }) => {
    return (
      <div className={`d-flex ${props.className}`}>
        <div className="flex-grow-1">{props.name}</div>
        <div>{props.value}</div>
      </div>
    );
  };

  const ListTitle = styled(ListRow)`
    color: #1572e8;
    font-size: 600;
    margin-bottom: 0.5rem;
  `;

  const [dataList, setDataList] = useState<[string, number][]>([]);
  useEffect(() => {
    fetchAptCount().then((v) => {
      setDataList(Object.entries(v));
    });
  }, []);

  return (
    <Card>
      <CardTitle>基本信息</CardTitle>
      <CardBody className="d-flex">
        <div className="flex-grow-1">
          <ListTitle
            name={"Total number of training samples:"}
            value={dataList.map((v) => v[1]).reduce((a, b) => a + b, 0)}
          />
          <div className="d-flex">
            <ul className="flex-grow-1">
              {dataList
                .slice(0, Math.floor(dataList.length / 2))
                .map(([name, value], i) => {
                  return (
                    <li key={`${name}${i}`}>
                      <ListRow name={name} value={value} />
                    </li>
                  );
                })}
            </ul>
            <Vr className="mx-3" />
            <ul className="flex-grow-1">
              {dataList
                .slice(Math.floor(dataList.length / 2))
                .map(([name, value], i) => {
                  return (
                    <li key={`${name}${i}`}>
                      <ListRow name={name} value={value} />
                    </li>
                  );
                })}
            </ul>
          </div>
        </div>
      </CardBody>
    </Card>
  );
};

const Md5Table = () => {
  type TableData = Map<string, string[]>;
  // key: name, value: md5[]
  const [tableData, setTableData] = useState<TableData>();

  useEffect(() => {
    fetchMd5TableData().then((v) => {
      const data = v.nodes.reduce((acc, curr) => {
        let val = acc.get(curr.name);
        if (val !== undefined) {
          val.push(curr.mdd);
        } else {
          acc.set(curr.name, [curr.mdd]);
        }
        return acc;
      }, new Map() as TableData);
      setTableData(data);
    });
  }, []);

  const Md5TableCell = (props: { md5: string }) => {
    const { md5 } = props;
    return (
      <td>
        <a
          href={`https://www.virustotal.com/gui/file/${md5}/detection`}
          target="_blank"
          rel="noreferrer"
        >
          {md5}
          <BsLink45Deg className="ms-1" />
        </a>
      </td>
    );
  };

  const table = tableData
    ? Array.from(tableData.entries()).map(([k, v], i) => {
        return (
          <React.Fragment key={i + k}>
            <tr>
              <td rowSpan={v.length}>{k}</td>
              <Md5TableCell md5={v[0]} />
            </tr>
            {v.slice(1).map((md5, ii) => (
              <tr key={ii + md5}>
                <Md5TableCell md5={md5} />
              </tr>
            ))}
          </React.Fragment>
        );
      })
    : null;

  return (
    <Card>
      <CardBody>
        <table
          className="table table-bordered mb-0"
          css={css`
            & a {
              text-decoration: none;
            }
          `}
        >
          <thead>
            <tr>
              <th>APT家族</th>
              <th>MD5</th>
            </tr>
          </thead>
          <tbody>{table}</tbody>
        </table>
      </CardBody>
    </Card>
  );
};

const DataView = () => {
  const [pieData, setPieData] = useState<PieData>();
  useEffect(() => {
    fetch(pieDataUrl)
      .then((res) => res.json())
      .then((v) => {
        setPieData(v as PieData);
      });
  }, []);

  return (
    <>
      <Navbar />
      <Container className="flex-grow-1 mb-4">
        <p className="fs-1 fw-light mt-2">数据总览</p>
        <hr />

        <BasicInfo />

        <Row className="my-4" style={{ height: "24rem" }}>
          <Col lg={6}>
            <PieCard label="Malware-Benign ratio" data={pieData?.first ?? []} />
          </Col>
          <Col lg={6}>
            <PieCard
              label="Sources of Benign files"
              data={pieData?.second ?? []}
            />
          </Col>
        </Row>
        <Md5Table />
      </Container>
      <Footer />
    </>
  );
};

export default DataView;
