import './main.scss';

import { Converter } from 'showdown';

(() => {
    // if ('serviceWorker' in navigator) {
    //     window.addEventListener('load', () => {
    //         navigator.serviceWorker.register('/service-worker.js').then(registration => {
    //             // tslint:disable:no-console
    //             console.log('SW registered: ', registration);
    //         }).catch(registrationError => {
    //             console.log('SW registration failed: ', registrationError);
    //         });
    //     });
    // }

    const appRoot = document.createElement('div');
    appRoot.classList.add('root');
    document.body.appendChild(appRoot);

    const debugTextEl = document.createElement('code');
    debugTextEl.classList.add('debug');
    document.body.appendChild(debugTextEl);

    const resolutions: any = {};

    const resolutionsProxy = new Proxy(resolutions, {set: (target, p, value, receiver) => {
        target[p] = value;
        debugTextEl.innerText = JSON.stringify(resolutions);
        return true;
    }, get: (target, property) => {
        return target[property];
    }});

    const context = {
        Intro: {
            1:  `
# Disclaimer!
This is not yet what it pretends to be. this is a design prototype

# absQra demo

In this demo we'll pretend you're trying an app that collects data about urban environment from first hand. It offers users a conscious moment in exchange for participation

## Have fun!

### For the fast runners
You have to complete it three times in this demo to go through all steps.

	`,
            ReturningUser: async (container: HTMLElement) => {
                return new Promise((resolve) => {
                    const button1 = document.createElement('button');
                    button1.innerText = 'let me in';
                    const button2 = document.createElement('button');
                    button2.innerText = 'try it first';
                    container.appendChild(button1);
                    container.appendChild(button2);
                    button1.onclick = () => {
                        resolve(true);
                    };

                    button2.onclick = () => {
                        resolve(false);
                    };
                });
            },
            3: 	`
You agree to participate...
I promise to delete your data upon request, if we are lucky and something is stored
`,
            Consent: async (container: HTMLElement) => {
                return new Promise((resolve) => {
                    const button1 = document.createElement('button');
                    button1.innerText = 'I agree';
                    const button2 = document.createElement('button');
                    button2.innerText = 'I do not agree';
                    container.appendChild(button1);
                    container.appendChild(button2);
                    button1.onclick = () => {
                        resolve(true);
                    };

                    button2.onclick = () => {
                        resolve(false);
                    };
                });
            }
        },
        Participation: {
            1: `Please enter your current location`,
            Location: async (container: HTMLElement) => {
                return new Promise((resolve) => {
                    const button1 = document.createElement('button');
                    button1.innerText = 'ENter location';
                    button1.onclick = () => {
                        resolve('ебать,геодатапошла');
                    };
                    container.appendChild(button1);
                })
            }
        },
        2: {},

        3: {
            1: `# Thank you!`
        }
    };

    const converter = new Converter();
    let skipped = false;
    async function skipToParticipation() {
        skipped = true;
        return stepIterativeParticipation(appRoot, context);
    }

    async function stepIterativeParticipation(container: HTMLElement, ctx: any) {
        const iterations = resolutionsProxy['IterativeParticipation'] || [];
        resolutionsProxy['IterativeParticipation'] = iterations;

        let i = resolutionsProxy['IterativeParticipation'].length;
        while (i < 2) {
            container.innerHTML = '';
            iterations[i] = await stepParticipation(container, ctx.Participation);
            resolutionsProxy['IterativeParticipation'] = iterations;
            i ++;
        }
        return resolutionsProxy['IterativeParticipation'];
    }

    async function stepTrial(container: HTMLElement, ctx: any) {
        resolutionsProxy['Participation'] = await stepParticipation(container, ctx.Participation);
    }

    async function stepIntro(container: HTMLElement, ctx: any) {
        async function step1(stepContainer: HTMLElement, stepCtx: any) {
            const html = converter.makeHtml(stepCtx[1]);
            const output1 = document.createElement('div');
            output1.innerHTML = html;
            stepContainer.appendChild(output1);

            const input1 = document.createElement('div');
            stepContainer.appendChild(input1);

            return stepCtx['ReturningUser'](input1);
        }

        async function innerStep2(stepContainer: HTMLElement, stepCtx: any) {
            const html = converter.makeHtml(stepCtx[3]);
            const output1 = document.createElement('div');
            output1.innerHTML = html;
            stepContainer.appendChild(output1);

            const input1 = document.createElement('div');
            stepContainer.appendChild(input1);
            resolutionsProxy['Consent'] = await stepCtx['Consent'](input1);
            return resolutions['Consent'];
        }

        const returningUser = await step1(container, ctx);
        container.innerHTML = '';

        if (returningUser) {
            console.log('go');
            resolutionsProxy.Consent = true;
            return skipToParticipation();
        }
        else {
            console.log('continue');
            return innerStep2(container, ctx);
        }
    }

    async function stepParticipation(container: HTMLElement, ctx: any) {
        const html = converter.makeHtml(ctx[1]);
        const output1 = document.createElement('div');
        output1.innerHTML = html;
        container.appendChild(output1);

        const input1 = document.createElement('div');
        container.appendChild(input1);

        return ctx['Location'](input1);
    }

    async function step2(container: HTMLElement, ctx: any) {
        if (resolutionsProxy['Consent']) {
            await stepTrial(container, context);
            return stepIterativeParticipation(container, context);
        }
        else {
            return;
        }
    }

    async function step3(container: HTMLElement, ctx: any) {
        const html = converter.makeHtml(ctx[1]);
        const output1 = document.createElement('div');
        output1.innerHTML = html;
        container.appendChild(output1);
    }

    async function runtime(container: HTMLElement, ctx: any) {
        await stepIntro(container, ctx.Intro);
        container.innerHTML = '';
        if (!skipped) {
            await step2(container, ctx[2]);
        }
        container.innerHTML = '';
        await step3(container, ctx[3]);
    }

    runtime(appRoot, context).catch((e) => {
        console.error(e);
    });
})();

class Runtime {

}

class DeliveryMedium {
    public readonly rootEl: HTMLElement;
    constructor(
        el: HTMLElement
    ) {
        this.rootEl = document.createElement('div');
        this.rootEl.setAttribute('id', 'delivery-root');
        el.appendChild(this.rootEl);
    }

    send(content: HTMLElement) {
        this.rootEl.innerHTML = '';
        this.rootEl.appendChild(content);
    }

    async request(req: (c: HTMLElement) => Promise<any>) {
        const reqEl = document.createElement('div');
        reqEl.setAttribute('id', 'request-root');
        this.send(reqEl);
        return req(reqEl);
    }
}

class Pair<T> {
    constructor(
       public readonly container: T
    ) {

    }

    send<I1, I2>(...items: [I1, I2]) {

    }
}
