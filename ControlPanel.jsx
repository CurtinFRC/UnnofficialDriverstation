const ControlPanel = ({ status }) => {
  return (
    <div>
      <h2 className="text-xl mb-2">Control Panel</h2>
      <div className="bg-green-500 text-black p-4 mb-2 rounded">
        <div className="flex justify-between items-center">
          <span>Robot Status:</span>
          <span>{status}</span>
        </div>
      </div>
      <div className="bg-gray-600 text-white p-4 rounded">
        <h3 className="text-lg mb-2">Status Indicators</h3>
        <div className="space-y-2">
          <div className="flex justify-between">
            <span>Robot Code:</span>
            <span className="text-red-500">Not Running</span>
          </div>
          <div className="flex justify-between">
            <span>Emergency Stop:</span>
            <span className="text-yellow-500">Inactive</span>
          </div>
          <div className="flex justify-between">
            <span>DS Radio:</span>
            <span className="text-red-500">Disconnected</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ControlPanel;

