const StatusIndicators = () => {
  return (
    <div className="flex flex-col w-1/3 p-4 bg-white shadow-md m-2">
      <h2 className="text-xl font-bold mb-4">Status Indicators</h2>
      <div className="flex flex-col space-y-2">
        <span>Robot Code: <span className="text-red-500">Not Running</span></span>
        <span>Emergency Stop: <span className="text-green-500">Inactive</span></span>
        <span>DS Radio: <span className="text-red-500">Disconnected</span></span>
      </div>
    </div>
  );
};

export { StatusIndicators };

