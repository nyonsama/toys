import { Container } from "react-bootstrap";
import { Card, CardBody, CardTitle } from "../components/Card";
import Footer from "../components/Footer";
import GraphContainer from "../components/GraphContainer";
import Navbar from "../components/Navbar";
import Vr from "../components/Vr";
import { geneGraphUrl } from "../lib/api";

const GeneGraph = () => {
  return (
    <>
      {" "}
      <Navbar />
      <Container className="flex-grow-1 mb-4">
        <p className="fs-1 fw-light mt-2">基因图谱</p>
        <hr />

        <Card>
          <CardBody className="d-flex">
            <div style={{ width: "24rem" }}>
              <p className="fs-3">Tips</p>
              <hr />
              <p>每个灰色大圈笼罩着一个家族</p>
              <p>每个彩色点代表一个样本</p>
              <p>
                每个灰色边代表家族间联系，粗细程度代表其联系强度，鼠标至于其上后可查看其联系程度得分(10分为联系程度最强)
              </p>
              <p>每个彩色边代表家族内部样本联系</p>
              <p>鼠标可以缩放大小，观看家族内部联系</p>
            </div>
            <Vr style={{ margin: "0 1rem" }} />

            <div
              style={{
                minWidth: "24rem",
                minHeight: "36rem",
                width: "100%",
              }}
            >
              <GraphContainer path={geneGraphUrl[0]} />
            </div>
          </CardBody>
        </Card>
        <div className="d-flex gap-4 mt-4">
          <Card className="w-50">
            <CardTitle>asdf</CardTitle>
            <CardBody style={{ height: "18rem" }}>
              <GraphContainer path={geneGraphUrl[1]} />
            </CardBody>
          </Card>

          <Card className="w-50">
            <CardTitle>asdf</CardTitle>
            <CardBody style={{ height: "18rem" }}>
              <GraphContainer path={geneGraphUrl[2]} />
            </CardBody>
          </Card>
        </div>
      </Container>
      <Footer />
    </>
  );
};

export default GeneGraph;
