import { src, dest } from 'gulp';
import * as fs from 'fs';
import * as path from 'path';
import * as exec from 'gulp-exec';
import * as merge from 'gulp-merge';

const paths = {
    src: 'src'
};

function getDirs(rootDir) {
    return fs.readdirSync(rootDir)
    .filter(function(file) {
        return fs.statSync(path.join(rootDir, file)).isDirectory();
    });
}

export function barrels() {
    const dirs = getDirs(paths.src);
    const commands = dirs.map((dir) => {
        return exec(`barrelsby -c=${path.join(paths.src, dir, 'barrelsby.conf.json')}`);
    });
    return merge(commands);
}