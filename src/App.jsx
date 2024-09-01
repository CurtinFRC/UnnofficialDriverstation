import { useState } from "preact/hooks";
import TopBar from "./components/topbar.jsx";
import { ControlPanel } from './components/controlpanel.jsx';
import DiagnosticsPanel from './components/diagnosticspanel.jsx'; // Ensure this matches the export name

// Main App Component
function App() {
  const [activeTab, setActiveTab] = useState("Home"); // State to manage active tab

  // Function to render content based on active tab
  const renderContent = () => {
    switch (activeTab) {
      case "Home":
        return <ControlPanel status="Active" />;
      case "Joysticks":
        return (
          <div className="p-4">
            <h2 className="text-xl mb-4">Joysticks</h2>
            {/* Add Joystick specific content here */}
            <p>Joystick status and settings will appear here.</p>
          </div>
        );
      case "Logs":
        return (
          <div className="p-4">
            <h2 className="text-xl mb-4">Logs</h2>
            {/* Add Log specific content here */}
            <p>Log information will appear here.</p>
          </div>
        );
      default:
        return <ControlPanel status="Active" />;
    }
  };

  return (
    <div className="bg-gray-900 text-white min-h-screen flex flex-col">
      {/* Top Bar */}
      <div className="bg-gray-800 flex justify-between items-center p-4">
        <TopBar teamNumber="0000" connectionStatus="Disconnected" />
        <div className="flex items-center space-x-4">
          <div className="text-lg">Elapsed Time: 00:00:00</div>
          <div className="text-lg">Team #: 40</div>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="bg-gray-800 p-2">
        <button
          onClick={() => setActiveTab("Home")}
          className={`px-4 py-2 ${
            activeTab === "Home" ? "bg-blue-500" : "bg-gray-700"
          } rounded`}
        >
          Home
        </button>
        <button
          onClick={() => setActiveTab("Joysticks")}
          className={`px-4 py-2 ${
            activeTab === "Joysticks" ? "bg-blue-500" : "bg-gray-700"
          } rounded ml-2`}
        >
          Joysticks
        </button>
        <button
          onClick={() => setActiveTab("Logs")}
          className={`px-4 py-2 ${
            activeTab === "Logs" ? "bg-blue-500" : "bg-gray-700"
          } rounded ml-2`}
        >
          Logs
        </button>
      </div>

      {/* Tab Content */}
      <div className="flex-grow p-4 bg-gray-700">
        {renderContent()}
      </div>
    </div>
  );
}

export default App;

