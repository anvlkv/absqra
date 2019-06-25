import * as Koa from 'koa';
import * as serve from 'koa-static';


const app = new Koa();

// app.use(async ctx => {
//     ctx.body = 'Hello 12345678';
// });

app.use(serve(`${__dirname}/beamer/dist`));

app.listen(3000);

console.log('AIR');
