// src/components/diagnosticspanel.jsx

const DiagnosticsPanel = () => (
  <div className="bg-white p-4 rounded-md shadow-md border border-gray-300">
    <h2 className="text-lg font-semibold mb-2">Diagnostics</h2>
    <div className="mb-4">
      <div className="flex justify-between mb-2">
        <span className="text-gray-700">Status:</span>
        <span className="font-bold text-red-500">Disconnected</span>
      </div>
      <div className="flex justify-between mb-2">
        <span className="text-gray-700">Battery:</span>
        <span className="font-bold text-green-500">12.5V</span>
      </div>
      <div className="flex justify-between">
        <span className="text-gray-700">Control:</span>
        <span className="font-bold text-gray-500">Inactive</span>
      </div>
    </div>
    <h3 className="text-lg font-semibold mb-2">Status Indicators</h3>
    <div className="flex flex-col">
      <div className="flex justify-between mb-2">
        <span className="text-gray-700">Robot Code:</span>
        <span className="font-bold text-red-500">Not Running</span>
      </div>
      <div className="flex justify-between mb-2">
        <span className="text-gray-700">Emergency Stop:</span>
        <span className="font-bold text-gray-500">Inactive</span>
      </div>
      <div className="flex justify-between">
        <span className="text-gray-700">DS Radio:</span>
        <span className="font-bold text-red-500">Disconnected</span>
      </div>
    </div>
  </div>
);

export default DiagnosticsPanel;  // Ensure this matches your import

