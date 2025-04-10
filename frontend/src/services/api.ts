import axios from 'axios';

const api = axios.create({
  baseURL: '/api', // Traefik will route this to backend
  withCredentials: true
});

export const healthCheck = async () => {
  return await api.get('/health');
};

export const createItem = async (itemData: any) => {
  return await api.post('/admin/items', itemData);
};

// Add more as needed...
