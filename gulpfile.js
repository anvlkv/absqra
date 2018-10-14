const gulp = require('gulp');
const $ = require('gulp-load-plugins')();
const log = require('fancy-log');
const print = require('gulp-print').default;

const src = {
	entity: 'intervey-api/lib/types/entity',
	apiEnums: 'intervey-api/src/entity/enums',
	routes: 'intervey-api/lib/router',
	clientDist: 'intervey-app/dist'
}

const dest = {
	apiModels: 'intervey-app/src/models/api-models',
	apiEnums: 'intervey-app/src/models/api-models/enums',
	apiRoutes: 'intervey-app/src/models/api-routes',
	clientDist: 'intervey-api/src/client'
};

gulp.task('clean:apiModels', () => {
	return gulp.src([`${dest.apiModels}/**/*.*`, `!${dest.apiEnums}/**/*.*`], {read: false})
		.pipe($.rm());
});

gulp.task('clean:apiRoutes', () => {
	return gulp.src(`${dest.apiRoutes}/**/*.*`, {read: false})
		.pipe($.rm());
});

gulp.task('clean:clientDist', () => {
	return gulp.src(`${dest.clientDist}/**/*.*`, {read: false})
		.pipe($.rm());
});

gulp.task('clean:apiEnums', () => {
	return gulp.src(`${dest.apiEnums}/**/*.*`, {read: false})
		.pipe($.rm());
});

gulp.task('clean', (cb) => {
	return $.sequence(['clean:apiModels', 'clean:apiRoutes', 'clean:clientDist'])(cb);
});

gulp.task('cp:apiModels', () => {
	const multipartEntityFilter = $.filter((file) => !/.*entity\.d\.ts/.test(file.path), {restore: true});
	return $.merge(
			gulp.src([`${src.entity}/**/*.*`, `!${src.entity}/enums/**/*.*`])
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
		.pipe(print())
		.pipe(gulp.dest(dest.apiModels));
});

gulp.task('cp:apiEnums', () => {
	return gulp.src(`${src.apiEnums}/**/*.*`)
		.pipe(print())
		.pipe($.replace('export declare enum', 'export enum'))
		.pipe(gulp.dest(dest.apiEnums));
});



gulp.task('cp:apiRoutes', () => {
	return gulp.src(`${src.routes}/**/*.*`)
		.pipe(print())
		.pipe(gulp.dest(dest.apiRoutes));
});

gulp.task('cp:clientDist', () => {
	return gulp.src(`${src.clientDist}/**/*.*`)
		.pipe($.filesize())
		.pipe(gulp.dest(dest.clientDist));
});

gulp.task('cp', ['cp:apiModels', 'cp:apiRoutes', 'cp:apiEnums', 'cp:clientDist']);

gulp.task('default', (cb) => {
	return $.sequence('clean', 'cp')(cb);
});

function createWatcher(path, tasks, delay = 1500) {
	let lastChangeTimeout;
	return $.watch(path, (e) => {
		if (lastChangeTimeout) {
			clearTimeout(lastChangeTimeout);
		}

		lastChangeTimeout = setTimeout(() => {
			$.sequence(...tasks)();
		}, delay);

		return e;
	});
}

gulp.task('watch:apiModels', (cb) => {
	return createWatcher([`${src.entity}/**/*.*`, `!${src.entity}/enums`], ['clean:apiModels', 'cp:apiModels']);
});

gulp.task('watch:clientDist', () => {
	return createWatcher(`${src.clientDist}/**/*.*`, ['clean:clientDist', 'cp:clientDist'], 2500);
});

gulp.task('watch:apiRoutes', () => {
	return createWatcher(`${src.routes}/**/*.*`, ['clean:apiRoutes', 'cp:apiRoutes']);
});

gulp.task('watch:apiEnums', () => {
	return createWatcher(`${src.apiEnums}/**/*.*`, ['clean:apiEnums', 'cp:apiEnums'], 2500)
})

gulp.task('watch', ['default', 'watch:apiModels', 'watch:apiRoutes', 'watch:clientDist']);

