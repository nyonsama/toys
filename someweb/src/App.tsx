import { BrowserRouter, Routes, Route } from "react-router-dom";
import Summary from "./routes/Summary";
import Home from "./routes/Home";
import GeneGraph from "./routes/GeneGraph";
import DataView from "./routes/DataView";
import LearningBoard from "./routes/LearningBoard";
import History from "./routes/History";

const App = () => {
  return (
    <div style={{ minHeight: "100vh" }} className="d-flex flex-column">
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="summary" element={<Summary />} />
          <Route path="genegraph" element={<GeneGraph />} />
          <Route path="dataview" element={<DataView />} />
          <Route path="learning" element={<LearningBoard />} />
          <Route path="history" element={<History />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
};
export default App;
