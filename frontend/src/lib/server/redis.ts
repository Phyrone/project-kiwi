import 'dotenv/config';

import redis_lib from 'redis';

export const redis = redis_lib.createClient({
	url: 'redis://localhost:6379/0',
	password: undefined
});
await redis.connect();
