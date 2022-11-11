import { Container } from "react-bootstrap";

const Footer = () => {
  return (
    <footer
      className="d-flex align-items-center bg-light"
      style={{ height: "4rem" }}
    >
      <Container className="d-flex justify-content-between">
        <div>APT Detection</div>
        <div>Copyright asdfzxcv</div>
      </Container>
    </footer>
  );
};

export default Footer;
