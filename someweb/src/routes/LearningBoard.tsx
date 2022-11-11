import _ from "lodash";
import { useEffect, useState } from "react";
import { Button, Container } from "react-bootstrap";
import { Card, CardBody, CardTitle } from "../components/Card";
import Footer from "../components/Footer";
import Navbar from "../components/Navbar";
import { fetchLearningBoardData, LearningBoardData } from "../lib/api";

import { Chart, registerables } from "chart.js";
import { Line } from "react-chartjs-2";

Chart.register(...registerables);

const LearningBoard = () => {
  const [learningBoardData, setLearningBoardData] =
    useState<LearningBoardData>();

  const options = {
    responsive: true,
    interaction: {
      intersect: false,
      mode: "index" as const,
    },
    plugins: {
      legend: {
        position: "top" as const,
      },
    },
  };

  useEffect(() => {
    fetchLearningBoardData().then((resp) => {
      setLearningBoardData(resp);
    });
  }, []);

  const labels = learningBoardData
    ? _.range(1, learningBoardData.train_acc_xy.length + 1)
    : [];

  const trainData = {
    labels,
    datasets: [
      {
        label: "train_acc",
        data: learningBoardData?.train_acc_xy,
        borderColor: "rgb(255, 99, 132)",
        backgroundColor: "rgba(255, 99, 132, 0.5)",
      },
      {
        label: "train_loss",
        data: learningBoardData?.train_loss_xy,
        borderColor: "rgb(53, 162, 235)",
        backgroundColor: "rgba(53, 162, 235, 0.5)",
      },
    ],
  };

  const valData = {
    labels,
    datasets: [
      {
        label: "val_acc",
        data: learningBoardData?.val_acc_xy,
        borderColor: "rgb(255, 99, 132)",
        backgroundColor: "rgba(255, 99, 132, 0.5)",
      },
      {
        label: "val_loss",
        data: learningBoardData?.val_loss_xy,
        borderColor: "rgb(53, 162, 235)",
        backgroundColor: "rgba(53, 162, 235, 0.5)",
      },
    ],
  };

  return (
    <>
      <Navbar />
      <Container className="flex-grow-1 mb-4">
        <p className="fs-1 fw-light mt-2">学习信息</p>
        <hr />
        <Card className="d-flex flex-column mb-4">
          <CardTitle>
            <div className="d-flex justify-content-between">
              <span>SAE Learning</span>
              <Button>Select Param</Button>
            </div>
          </CardTitle>
          <CardBody className="flex-grow-1">
            <Line options={options} data={trainData} />
          </CardBody>
        </Card>

        <Card className="d-flex flex-column">
          <CardTitle>
            <div className="d-flex justify-content-between">
              <span>SAE Learning</span>
              <Button>Select Param</Button>
            </div>
          </CardTitle>
          <CardBody className="flex-grow-1">
            <Line options={options} data={valData} />
          </CardBody>
        </Card>
      </Container>
      <Footer />
    </>
  );
};

export default LearningBoard;
