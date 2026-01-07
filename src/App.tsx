import { Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import Home from './pages/Home';
import Projects from './pages/Projects';
import Logbook from './pages/Logbook';
import About from './pages/About';

function App() {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/projects" element={<Projects />} />
        <Route path="/log" element={<Logbook />} />
        <Route path="/about" element={<About />} />
      </Routes>
    </Layout>
  );
}

export default App;
