// src/components/DriverOperatorControl.jsx
import { useState, useEffect } from 'preact/hooks';

function DriverOperatorControl() {
  const [driverControls, setDriverControls] = useState({});
  const [operatorControls, setOperatorControls] = useState({});

  useEffect(() => {
    const fetchData = async () => {
      const response = await fetch('/api/driver-operator-controls');
      const data = await response.json();
      setDriverControls(data.driverControls);
      setOperatorControls(data.operatorControls);
    };

    fetchData();
  }, []);

  return (
    <div className="p-4 border border-gray-300 rounded">
      <h2 className="text-xl font-bold">Driver and Operator Control</h2>
      <div className="mt-4">
        <h3 className="text-lg">Driver Controls:</h3>
        <pre>{JSON.stringify(driverControls, null, 2)}</pre>
      </div>
      <div className="mt-4">
        <h3 className="text-lg">Operator Controls:</h3>
        <pre>{JSON.stringify(operatorControls, null, 2)}</pre>
      </div>
    </div>
  );
}

export default DriverOperatorControl;

