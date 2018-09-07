const cpy = require('recursive-copy');

const del = require('delete');

const src = {
	entity: 'intervey-api/lib/types/entity',
	enums: 'intervey-api/src/entity/enums',
	clientDist: 'intervey-app/dist',
	routes: 'intervey-api/lib/router'
}

const dest = {
	apiModels: 'intervey-app/src/models/api-models',
	apiEnums: 'intervey-app/src/models/api-models/enums',
	apiRoutes: 'intervey-app/src/models/api-routes',
	clientDist: 'intervey-api/src/client'
};

del([`${dest.apiModels}/**/*.*`, `${dest.apiRoutes}/**/*.*` , `${dest.clientDist}/**/*.*`], (err) => {
	if(err){
		console.error(err);
	}
	else {
		console.log('clean dest');

		cpy(src.entity, dest.apiModels, {
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

		cpy(src.enums, dest.apiEnums, {
			rename: basename => basename.replace('.d.ts', '.ts')
		}).then(() => {
			console.log('copied enums');
		}).catch(()=>{
			console.error('waiting for enums');
		});

		cpy(src.clientDist, dest.clientDist).then(() => {
			console.log('copied client dist');
		}).catch(()=>{
			console.error('waiting for client dist');
		});

		cpy(src.routes, dest.apiRoutes).then(() => {
			console.log('copied api routes');
		}).catch(()=>{
			console.error('waiting for api routes');
		});
	}
});

