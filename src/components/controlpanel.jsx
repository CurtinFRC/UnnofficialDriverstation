const ControlPanel = () => {
  return (
    <div className="flex flex-col w-1/3 p-4 bg-white shadow-md m-2">
      <h2 className="text-xl font-bold mb-4">Control Panel</h2>
      <button className="bg-green-500 text-white p-2 rounded mb-2">Enable</button>
      <button className="bg-red-500 text-white p-2 rounded mb-2">Disable</button>
      <div className="mt-4">
        <label className="block mb-2">Mode:</label>
        <select className="border p-2 w-full">
          <option>Teleop</option>
          <option>Autonomous</option>
          <option>Test</option>
        </select>
      </div>
    </div>
  );
};

export { ControlPanel };

