import 'dotenv/config';
import { Client } from 'minio';


export const minio = new Client({
	endPoint: 'localhost',
	port: 9000,
	accessKey: '8vvOWN2qCboZRtw1fcJQ',
	secretKey: 'NJdzWNwiNO2ZU92pWzcA5rUE1eqOjM3RGpr1rtdW',
	useSSL: false
});