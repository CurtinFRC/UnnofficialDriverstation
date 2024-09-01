// src/components/StatusIndicators.jsx
import { useState, useEffect } from 'preact/hooks';

function StatusIndicators() {
  const [communicationStatus, setCommunicationStatus] = useState('Disconnected');
  const [batteryVoltage, setBatteryVoltage] = useState('N/A');

  useEffect(() => {
    const fetchData = async () => {
      const response = await fetch('/api/status');
      const data = await response.json();
      setCommunicationStatus(data.communicationStatus);
      setBatteryVoltage(data.batteryVoltage);
    };

    fetchData();
  }, []);

  return (
    <div className="p-4 border border-gray-300 rounded">
      <h2 className="text-xl font-bold">Status Indicators</h2>
      <div className="mt-4">
        <h3 className="text-lg">Robot Communication:</h3>
        <p>{communicationStatus}</p>
      </div>
      <div className="mt-4">
        <h3 className="text-lg">Battery Voltage:</h3>
        <p>{batteryVoltage} V</p>
      </div>
    </div>
  );
}

export default StatusIndicators;

