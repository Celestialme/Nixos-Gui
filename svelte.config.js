import adapter from '@sveltejs/adapter-static'
import preprocess from 'svelte-preprocess';
import Path from 'path';
/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),
			vite:{
			server:{fs:{allow:['static']}},
			resolve:{
				alias:{
					'@src':Path.resolve('src/'),
					'@static':Path.resolve('static/')
				}
			},
			

		},
		prerender:{default:true},
	}
};

export default config;
