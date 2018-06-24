const cpy = require('recursive-copy');

const del = require('delete');

const dest = {
	apiModels: 'intervey-app/src/api-models',
	apiEnums: 'intervey-app/src/api-models/enums',
	apiRoutes: 'intervey-app/src/api-routes',
	clientDist: 'intervey-api/src/client'
};

del([`${dest.apiModels}/**/*.*`, `${dest.apiRoutes}/**/*.*` , `${dest.clientDist}/**/*.*`], (err) => {
	if(err){
		console.error(err);
	}
	else {
		console.log('clean dest');

		cpy('intervey-api/lib/types/entity', dest.apiModels, {
			filter: [
				'*.ts',
				/!enums\/*/,
				/!index.*/g
			],
			rename: basename => basename.replace('.d.ts', '.ts'),
		}).then(() => {
			console.log('copied entities');
		}).catch(()=>{
			console.error('waiting for entities');
		});

		cpy('intervey-api/src/entity/enums', dest.apiEnums, {
			rename: basename => basename.replace('.d.ts', '.ts')
		}).then(() => {
			console.log('copied enums');
		}).catch(()=>{
			console.error('waiting for enums');
		});

		cpy('intervey-app/dist', dest.clientDist).then(() => {
			console.log('copied client dist');
		}).catch(()=>{
			console.error('waiting for client dist');
		});

		cpy('intervey-api/lib/router', dest.apiRoutes).then(() => {
			console.log('copied api routes');
		}).catch(()=>{
			console.error('waiting for api routes');
		});
	}
});

