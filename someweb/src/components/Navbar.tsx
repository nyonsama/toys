import { Container, Nav, Navbar as BsNavbar } from "react-bootstrap";
import { Link } from "react-router-dom";

const Navbar = () => {
  return (
    <BsNavbar bg="light" expand="lg">
      <Container>
        <BsNavbar.Brand>APT Detection</BsNavbar.Brand>
        <BsNavbar.Toggle />
        <BsNavbar.Collapse>
          <Nav className="me-auto">
            <Link className="nav-link" to="/">
              主页
            </Link>
            <Link className="nav-link" to="/summary">
              分析结果
            </Link>
            <Link className="nav-link" to="/genegraph">
              基因图谱
            </Link>
            <Link className="nav-link" to="/learning">
              学习信息
            </Link>
            <Link className="nav-link" to="/dataview">
              数据总览
            </Link>
            <Link className="nav-link" to="/history">
              历史记录
            </Link>
          </Nav>
        </BsNavbar.Collapse>
      </Container>
    </BsNavbar>
  );
};

export default Navbar;
