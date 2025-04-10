import React, { useEffect, useState } from 'react'
import axios from 'axios'

const Terminal: React.FC = () => {
  const [status, setStatus] = useState("Loading...")

  useEffect(() => {
    axios.get("/api/health")
      .then(res => {
        if (res.status === 200) {
          setStatus("✅ Backend is online.")
        } else {
          setStatus("⚠️ Backend healthcheck failed.")
        }
      })
      .catch(() => setStatus("❌ Cannot connect to backend."))
  }, [])

  return (
    <div>
      <pre>{status}</pre>
    </div>
  )
}

export default Terminal
