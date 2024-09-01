// src/index.jsx
import { render } from 'preact';
import ControlPanel from './components/ControlPanel';
import StatusIndicators from './components/StatusIndicators';
import MatchControls from './components/MatchControls';
import DriverOperatorControl from './components/DriverOperatorControl';
import Diagnostics from './components/Diagnostics';
import NetworkConfiguration from './components/NetworkConfiguration';
import './tailwind.css';

function App() {
  return (
    <div className="app">
      <ControlPanel />
      <StatusIndicators />
      <MatchControls />
      <DriverOperatorControl />
      <Diagnostics />
      <NetworkConfiguration />
    </div>
  );
}

render(<App />, document.getElementById('root'));

