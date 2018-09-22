const gulp = require('gulp');
const $ = require('gulp-load-plugins')();
const log = require('fancy-log');

const src = {
	entity: 'intervey-api/lib/types/entity',
	clientDist: 'intervey-app/dist',
	routes: 'intervey-api/lib/router',
	apiEnums: 'intervey-api/src/entity/enums'
}

const dest = {
	apiModels: 'intervey-app/src/models/api-models',
	apiEnums: 'intervey-app/src/models/api-models/enums',
	apiRoutes: 'intervey-app/src/models/api-routes',
	clientDist: 'intervey-api/src/client'
};

gulp.task('clear', () => {
	return gulp.src([
			`${dest.apiModels}/**/*.*`,
			`${dest.apiRoutes}/**/*.*` ,
			`${dest.clientDist}/**/*.*`
		], { read: false })
		.pipe($.rm());
});

gulp.task('cp:entity', () => {
	const multipartEntityFilter = $.filter((file) => !/.*entity\.d\.ts/.test(file.path), {restore: true});

	return $.merge(
			gulp.src(`${src.entity}/**/*.*`)
				.pipe($.filter((file) => !/.*enums\.d\.ts/.test(file.path)))
				.pipe($.filter((file) => !/.*transform\.d\.ts/.test(file.path)))
				.pipe($.rename((path) => {
					path.basename = path.basename.replace(/\.d$/, '');
					if (path.basename === 'entity') {
						path.basename = path.dirname;
						path.dirname = '.';
					}
				}))
				.pipe($.replace(/\/entity.*/gm, '\';'))
				.pipe($.replace(/^(.*)(\/transform.*)/gm, ''))
				.pipe(multipartEntityFilter)
				.pipe($.replace(/(\.\.\/)(\w)/gm, './$2')),
			multipartEntityFilter.restore
		)
		.pipe(gulp.dest(dest.apiModels));
});

gulp.task('cp:enums', () => {
	return gulp.src(`${src.apiEnums}/**/*.*`)
		.pipe(gulp.dest(dest.apiEnums));
});



gulp.task('cp:apiRoutes', () => {
	return gulp.src(`${src.routes}/**/*.*`)
		.pipe(gulp.dest(dest.apiRoutes));
});

gulp.task('cp:clientDist', () => {
	return gulp.src(`${src.clientDist}/**/*.*`)
		.pipe(gulp.dest(dest.clientDist));
});

gulp.task('cp', ['cp:entity', 'cp:apiRoutes', 'cp:enums', 'cp:clientDist']);

gulp.task('default', (cb) => {
	return $.sequence('clear', 'cp')(cb)
});