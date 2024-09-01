// src/components/MatchControls.jsx
function MatchControls() {
  const startMatch = async () => {
    await fetch('/api/start-match', { method: 'POST' });
  };

  const stopMatch = async () => {
    await fetch('/api/stop-match', { method: 'POST' });
  };

  const setAutonomousMode = async () => {
    await fetch('/api/set-mode', {
      method: 'POST',
      body: JSON.stringify({ mode: 'autonomous' }),
    });
  };

  const setTeleopMode = async () => {
    await fetch('/api/set-mode', {
      method: 'POST',
      body: JSON.stringify({ mode: 'teleop' }),
    });
  };

  return (
    <div className="p-4 border border-gray-300 rounded">
      <h2 className="text-xl font-bold">Match Controls</h2>
      <div className="mt-4">
        <button onClick={startMatch} className="bg-blue-500 text-white py-2 px-4 rounded">
          Start Match
        </button>
        <button onClick={stopMatch} className="bg-red-500 text-white py-2 px-4 rounded ml-4">
          Stop Match
        </button>
      </div>
      <div className="mt-4">
        <button onClick={setAutonomousMode} className="bg-green-500 text-white py-2 px-4 rounded">
          Set Autonomous Mode
        </button>
        <button onClick={setTeleopMode} className="bg-yellow-500 text-white py-2 px-4 rounded ml-4">
          Set Teleop Mode
        </button>
      </div>
    </div>
  );
}

export default MatchControls;

