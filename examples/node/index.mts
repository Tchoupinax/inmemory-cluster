import { createClient } from 'redis';

const redisClient = createClient({ url: 'redis://localhost:6379' });
redisClient.on('error', (err) => console.log('Redis Client Error', err));

await redisClient.connect();

export const setValue = async (key: string, value: string): Promise<void> => {
  await redisClient.set(key, value);
};

export const getValue = async (key: string): Promise<string | null> => {
  return redisClient.get(key);
};

for (let i = 0; i < 1; i++) {
    await setValue(i.toString(), "coucou");
}

for (let i = 0; i < 1; i++) {
    console.log(`Result: ${i}-${await getValue(i.toString())}`)
}
