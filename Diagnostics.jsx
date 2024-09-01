const DiagnosticsPanel = ({ status, battery, control }) => {
  return (
    <div>
      <h2 className="text-xl mb-4">Diagnostics</h2>
      <div className="space-y-2">
        <div className="flex justify-between">
          <span>Status:</span>
          <span className="text-red-500">{status}</span>
        </div>
        <div className="flex justify-between">
          <span>Battery:</span>
          <span className="text-green-500">{battery}</span>
        </div>
        <div className="flex justify-between">
          <span>Control:</span>
          <span className="text-yellow-500">{control}</span>
        </div>
      </div>
    </div>
  );
};

export default DiagnosticsPanel;

