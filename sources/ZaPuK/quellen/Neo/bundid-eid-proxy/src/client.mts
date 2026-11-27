#!/usr/bin/env node

/*
 * SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

import { createServer } from 'node:http';
import { spawnSync } from 'node:child_process';

if (process.argv.length !== 3) {
    console.log('usage: client.mts <browser command>');
    console.log();
    console.log('example: client.mts firefox');
    process.exit(1);
}

const browserCommand = process.argv[2];

const client = createServer(async (request, response) => {
    const requestHeader = `${request.method} ${request.url}`;

    console.log(`-> ${requestHeader.substring(0, 100)}...`);

    if (requestHeader.startsWith('GET /eID-Client')) {
        response.writeHead(200, 'Ok');
        response.end();

        const requestUrl = new URL(request.url!, 'http://localhost:1235');
        const tcTokenURL = requestUrl.searchParams.get('tcTokenURL')!;

        const eIDUrl = new URL('http://127.0.0.1:24727/eID-Client');
        eIDUrl.searchParams.set('tcTokenURL', tcTokenURL);

        console.log(`   GET ${eIDUrl.href.substring(0, 100)}...`);

        // NOTE: AusweisApp implements a broken webserver that node.js doesn't want to talk to.
        //       See https://chaos.social/@networkexception/116200406637837268
        const refreshUrl = new URL(spawnSync('curl', [ '-w%header{location}', eIDUrl.href ]).stdout.toString());

        const eIDResponseUrl = new URL('http://localhost:1234');
        eIDResponseUrl.pathname = refreshUrl.pathname;
        eIDResponseUrl.search = refreshUrl.search;

        console.log(`   ${browserCommand} ${eIDResponseUrl.href}`);
        console.log();

        spawnSync(browserCommand, [ eIDResponseUrl.href ]).stdout.toString();
    }
});

client.listen(1235);
