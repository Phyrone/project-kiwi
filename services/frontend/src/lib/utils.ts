import ms from 'ms';


export async function delay(delay: number | string) {
	let duration: number;
	if (typeof delay === 'string')
		duration = ms(delay);
	else {
		duration = delay;
	}

	return new Promise(resolve => setTimeout(resolve, duration));
}