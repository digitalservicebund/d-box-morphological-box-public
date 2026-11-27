#!/usr/bin/env node

/*
 * SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

import { createServer, IncomingMessage, ServerResponse } from 'node:http';
import diagnostics from 'node:diagnostics_channel'
import path from 'node:path';
import stream from 'node:stream/consumers';
import querystring from 'node:querystring';

import undici from 'undici';

import { CookieJar } from 'tough-cookie';
import { CookieAgent } from 'http-cookie-agent/undici';
import { readFileSync } from 'node:fs';
import html from 'node-html-parser';

const projectRoot = path.dirname(import.meta.dirname);

if (process.argv.length !== 4) {
    console.log('usage: server.mts <BundID base> <eID base>');
    console.log();
    console.log('example: server.mts https://int.id.bund.de http://localhost:1235');
    process.exit(1);
}

process.on('SIGINT', () => process.exit(0));

const bundIDBase = process.argv[2];
const eIDBase = process.argv[3];

const server = createServer(async (request, response) => {
    const requestHeader = `${request.method} ${request.url}`;

    if (requestHeader === 'POST /idp/profile/SAML2/POST/SSO') {
        console.log('->', requestHeader);

        await spinner(request, response);
        await handleSAMLRequest(request, response);
        return;
    } else if (requestHeader.startsWith('GET /idp/externalNpaAuthn')) {
        console.log('->', requestHeader);

        await spinner(request, response);
        await handleEIDResponse(request, response);
        return;
    } else if (requestHeader === 'GET /assets/index.css') {
        response.writeHead(200, 'Ok', { 'Content-Type': 'text/css; charset=UTF-8' });
        response.end(readFileSync(path.join(projectRoot, 'assets/index.css')));
        return;
    } else if (requestHeader === 'GET /assets/LoadingSpinner.css') {
        response.writeHead(200, 'Ok', { 'Content-Type': 'text/css; charset=UTF-8' });
        response.end(readFileSync(path.join(projectRoot, 'assets/LoadingSpinner.css')));
        return;
    } else if (requestHeader === 'GET /assets/AppLayout.css') {
        response.writeHead(200, 'Ok', { 'Content-Type': 'text/css; charset=UTF-8' });
        response.end(readFileSync(path.join(projectRoot, 'assets/AppLayout.css')));
        return;
    } else if (requestHeader === 'GET /healthz') {
        response.writeHead(200, 'Ok', { 'Content-Type': 'text/plain; charset=UTF-8' });
        response.end("OK");
        return;
    }

    response.writeHead(404, 'Not found');
    response.end();
});

server.listen(1234);

console.log(`Started BundID eID Proxy to ${bundIDBase} (via ${eIDBase})`);

diagnostics.channel('undici:request:create').subscribe((message: any) => {
    const request = message.request as undici.DiagnosticsChannel.Request;

    // console.log(`   ${request.method} ${request.origin}${request.path} (${request.headers})`);
    console.log(`   ${request.method} ${request.origin}${request.path}`);
});

async function spinner(request: IncomingMessage, response: ServerResponse<IncomingMessage>): Promise<void> {
    console.log(`<- HTTP/1.1 200 OK <html><body><div class="loading-spinner">...`);

    response.writeHead(200, {
        'Content-Type': 'text/html; charset=UTF-8',
        'Transfer-Encoding': 'chunked'
    });

    response.write(`
        <!DOCTYPE html>
        <html>
        <head>
            <link rel="stylesheet" crossorigin href="/assets/AppLayout.css">
            <link rel="stylesheet" crossorigin href="/assets/LoadingSpinner.css">
            <link rel="stylesheet" crossorigin href="/assets/index.css">
        </head>
        <body>
            <div class="app-layout" style="justify-content: center;">
                <div class="loading-spinner" role="status" aria-live="polite">
                    <span class="loading-spinner__indicator" aria-hidden="true"></span>
                    <span class="loading-spinner__text"></span>
                </div>
            </div>
    `);
}

async function handleSAMLRequest(request: IncomingMessage, response: ServerResponse<IncomingMessage>): Promise<void> {
    const cookieJar = new CookieJar();
    // @ts-expect-error FIXME: Resolve type conflict.
    const dispatcher = new CookieAgent({ cookies: { jar: cookieJar } });

    const ssoResponse = await undici.request(new URL('/idp/profile/SAML2/POST/SSO', bundIDBase), {
        dispatcher,
        method: 'POST',
        body: request,
        headers: {
            // NOTE: node fetch doesn't allow setting this header but BundID requires it.
            //       See https://github.com/nodejs/undici/issues/1305
            'Sec-Fetch-Mode': 'navigate',
            'Sec-Fetch-Site': 'cross-site',
            'Sec-Fetch-Dest': 'document',

            'Content-Type': 'application/x-www-form-urlencoded',
        },
    });

    const ssoResponseRedirect = new URL(ssoResponse.headers['location'] as string, bundIDBase);
    const ssoExecutionResponse = await undici.request(ssoResponseRedirect.href, {
        dispatcher,
        method: 'GET',
        headers: {
            'Sec-Fetch-Mode': 'navigate',
            'Sec-Fetch-Site': 'cross-site',
            'Sec-Fetch-Dest': 'document',
        },
    });

    const ssoExecutionForm = html.parse(await stream.text(ssoExecutionResponse.body));
    const jwt = ssoExecutionForm.querySelector('input[name=jwt]')?.getAttribute('value');

    const fachverfahrenResponse = await undici.request(new URL('/api/v1/nutzerkonto-management/public/fachverfahren', bundIDBase), {
        dispatcher,
        method: 'POST',
        body: querystring.encode({ jwt }),
        headers: {
            'Sec-Fetch-Mode': 'navigate',
            'Sec-Fetch-Site': 'cross-site',
            'Sec-Fetch-Dest': 'document',

            'Content-Type': 'application/x-www-form-urlencoded',
        },
    });

    await undici.request(`${fachverfahrenResponse.headers['location']}`, {
        dispatcher,
        method: 'GET',
        headers: {
            'Sec-Fetch-Mode': 'navigate',
            'Sec-Fetch-Site': 'cross-site',
            'Sec-Fetch-Dest': 'document',
        },
    });

    const skCsrfTokenRepsonse = await undici.request(new URL('/api/sk-csrf-token', bundIDBase), {
        dispatcher,
        method: 'GET',
        headers: {
            'Accept': 'application/json',
        },
    });

    const xCsrfToken = skCsrfTokenRepsonse.headers['x-csrf-token'];

    const eIDInitResponse = await undici.request(new URL('/api/v1/nutzerkonto-management/public/login/eid', bundIDBase), {
        dispatcher,
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',

            'X-XSRF-TOKEN': xCsrfToken,
        },
        body: JSON.stringify({
            redirectUrl: '/de/redirect',
            isGuestLogin: false,
        }),
    });

    const eIDInitUrl = new URL((await stream.json(eIDInitResponse.body) as any).url);
    const tcTokenURL = eIDInitUrl.searchParams.get('tcTokenURL')!;

    const redirect = new URL('/eID-Client', eIDBase);
    redirect.searchParams.set('tcTokenURL', tcTokenURL);

    console.log(`<- <script>window.location.href = "${redirect.href.substring(0, 70)}...";</script></body></html> EOF`);
    console.log();

    const cookies = await cookieJar.getCookies(bundIDBase, { allPaths: true });
    for (const cookie of cookies) {
        cookie.httpOnly = false;
    }

    response.end(`
            <script>
                const cookies = ${JSON.stringify(cookies.map(cookie => cookie.toString()))};

                for (const cookie of cookies) {
                    document.cookie = cookie;
                }

                window.location.href = ${JSON.stringify(redirect.href)};
            </script>
        </body>
    `);
}

async function handleEIDResponse(request: IncomingMessage, response: ServerResponse<IncomingMessage>): Promise<void> {
    const requestUrl = new URL(request.url!, 'http://localhost:1234');

    const cookieJar = new CookieJar();

    for (const cookie of request.headers.cookie?.split(';') ?? []) {
        await cookieJar.setCookie(cookie, bundIDBase);
    }

    // @ts-expect-error FIXME: Resolve type conflict.
    const dispatcher = new CookieAgent({ cookies: { jar: cookieJar } });

    const bundIDRefreshResponse = await (async () => {
        while (true) {
            try {
                const bundIDRefreshUrl = new URL('/idp/externalNpaAuthn', bundIDBase);
                bundIDRefreshUrl.search = requestUrl.search;

                return await undici.request(bundIDRefreshUrl, {
                    dispatcher,
                    method: 'GET',
                    headers: {
                        'Sec-Fetch-Mode': 'navigate',
                        'Sec-Fetch-Site': 'cross-site',
                        'Sec-Fetch-Dest': 'document',
                        'Sec-Fetch-User': '?1',

                        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:148.0) Gecko/20100101 Firefox/148.0',
                        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
                        'Accept-Language': 'en-US,en;q=0.9',
                        'Sec-GPC': '1',
                        'Connection': 'keep-alive',
                        'Upgrade-Insecure-Requests': '1',
                        'Priority': 'u=0, i',
                        'Pragma': 'no-cache',
                        'Cache-Control': 'no-cache'
                    }
                });
            } catch (e) {
                console.log('   Error', e);
                await new Promise(resolve => setTimeout(resolve, 3 * 1000));
            }
        }
    })();

    const bundIDRefreshResponseForm = html.parse(await stream.text(bundIDRefreshResponse.body));
    const RelayState = bundIDRefreshResponseForm.querySelector('input[name=RelayState]')?.getAttribute('value');
    const SAMLResponse = bundIDRefreshResponseForm.querySelector('input[name=SAMLResponse]')?.getAttribute('value');

    const ssoIdpResponse = await undici.request(new URL('/login/saml2/sso/idp', bundIDBase), {
        dispatcher,
        method: 'POST',
        body: querystring.encode({ RelayState, SAMLResponse }),
        headers: {
            'Sec-Fetch-Mode': 'navigate',
            'Sec-Fetch-Site': 'cross-site',
            'Sec-Fetch-Dest': 'document',

            'Content-Type': 'application/x-www-form-urlencoded',
        },
    });

    await undici.request(new URL(ssoIdpResponse.headers['location'] as string, bundIDBase), {
        dispatcher,
        method: 'GET',
        headers: {
            'Sec-Fetch-Mode': 'navigate',
            'Sec-Fetch-Site': 'cross-site',
            'Sec-Fetch-Dest': 'document',
        },
    });

    const skCsrfTokenRepsonse = await undici.request(new URL('/api/sk-csrf-token', bundIDBase), {
        dispatcher,
        method: 'GET',
        headers: {
            'Accept': 'application/json',
        },
    });

    const xCsrfToken = skCsrfTokenRepsonse.headers['x-csrf-token'];

    const consentResponse = await undici.request(new URL('/api/v1/nutzerkonto-management/public/fachverfahren/consent', bundIDBase), {
        dispatcher,
        method: 'POST',
        headers: {
            'Accept': 'application/json',

            'X-XSRF-TOKEN': xCsrfToken,
        }
    });

    const consentResponseJSON = (await stream.json(consentResponse.body)) as any;

    console.log(`<- form.submit("${consentResponseJSON.url.substring(0, 50)}...");</script></html> EOF`);
    console.log();

    response.end(`
            <script>
                const url = ${JSON.stringify(consentResponseJSON.url)};
                const params = ${JSON.stringify(consentResponseJSON.params)};

                const form = document.createElement("form");
                form.method = "POST";
                form.action = url;

                for (const key in params) {
                    const input = document.createElement("input");
                    input.name = key;
                    input.value = params[key];
                    input.type = "hidden";

                    form.appendChild(input);
                }

                document.body.appendChild(form);

                form.submit();
            </script>
        </body>
    `);
}
